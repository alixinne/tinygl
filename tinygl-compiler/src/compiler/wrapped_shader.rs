use std::path::{Path, PathBuf};

use heck::{CamelCase, SnakeCase};

use quote::{format_ident, quote};

use rspirv::dr as rr;

use super::{TargetType, WrappedItem};
use crate::shader_kind::ShaderKindInfo;
use crate::types::prelude::*;

pub struct WrappedShader {
    shader: String,
    rs_file_name: String,
    uniforms: Vec<crate::reflect::FoundUniform>,
    kind: ShaderKindInfo,
    source_path: PathBuf,

    shader_struct_name: String,
    shader_variable_name: String,
    uniform_struct_name: String,
    uniform_locations_name: String,

    binary_result: shaderc::CompilationArtifact,
    output_type: TargetType,
    skip_spirv: bool,
}

impl std::fmt::Debug for WrappedShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappedShader")
            .field("shader", &self.shader)
            .field("rs_file_name", &self.rs_file_name)
            .field("uniforms", &self.uniforms)
            .field("kind", &self.kind)
            .field("source_path", &self.source_path)
            .field("output_type", &self.output_type)
            .field("skip_spirv", &self.skip_spirv)
            .finish()
    }
}

impl WrappedShader {
    pub fn new(
        kind: ShaderKindInfo,
        source_path: &Path,
        binary_result: shaderc::CompilationArtifact,
        output_type: TargetType,
        skip_spirv: bool,
    ) -> Self {
        let shader: String = source_path.file_name().unwrap().to_string_lossy().into();

        let base_name = shader.replace(".", "_");
        let shader_struct_name = (base_name.to_owned() + "_shader").to_camel_case();
        let shader_variable_name = shader_struct_name.to_snake_case();

        Self {
            shader,
            rs_file_name: base_name.to_owned() + ".rs",
            uniforms: Vec::new(),
            kind,
            source_path: source_path.to_owned(),
            shader_struct_name,
            shader_variable_name,
            uniform_struct_name: (base_name.to_owned() + "_uniforms").to_camel_case(),
            uniform_locations_name: (base_name + "_locations").to_snake_case(),
            binary_result,
            output_type,
            skip_spirv,
        }
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    pub fn uniforms(&self) -> &[crate::reflect::FoundUniform] {
        &self.uniforms[..]
    }

    pub fn shader_struct_name(&self) -> &str {
        &self.shader_struct_name
    }

    pub fn shader_variable_name(&self) -> &str {
        &self.shader_variable_name
    }

    pub fn uniform_struct_name(&self) -> &str {
        &self.uniform_struct_name
    }

    pub fn uniform_locations_name(&self) -> &str {
        &self.uniform_locations_name
    }

    pub fn reflect_uniforms(&mut self) -> crate::Result<()> {
        // Extract uniform data
        let mut loader = rr::Loader::new();
        rspirv::binary::parse_words(self.binary_result.as_binary(), &mut loader).unwrap_or_else(
            |_| {
                panic!(
                    "failed to parse binary module for {}",
                    self.source_path.to_string_lossy()
                )
            },
        );

        self.uniforms =
            crate::reflect::find_uniforms(&self.source_path.to_string_lossy(), &loader.module())?;

        Ok(())
    }

    fn write_shader(&self) -> crate::Result<proc_macro2::TokenStream> {
        match self.output_type {
            TargetType::SpirV => {
                // Just write spv file
                let res = syn::LitByteStr::new(
                    &self.binary_result.as_binary_u8(),
                    proc_macro2::Span::call_site(),
                );
                Ok(quote! { #res })
            }
            TargetType::Glsl(version) => {
                let out = if self.skip_spirv {
                    use std::fmt::Write;

                    // We skipped SPIR-V generation so just fix invalid stuff for OpenGL ES targets
                    // WebGL is more sensitive to leftovers from includes and stuff
                    // TODO: This is an ugly hack, maybe forbid skip_spirv + ES 3.00?
                    let mut output = String::new();

                    for l in self.binary_result.as_text().lines() {
                        if l.starts_with("#extension GL_GOOGLE_include_directive") {
                            continue;
                        } else if l.starts_with("#line") {
                            writeln!(output, "//{}", l).ok();
                        } else {
                            writeln!(output, "{}", l).ok();
                        }
                    }

                    output
                } else {
                    // Use spirv_cross to write valid code
                    let module =
                        spirv_cross::spirv::Module::from_words(self.binary_result.as_binary());
                    let mut ast =
                        spirv_cross::spirv::Ast::<spirv_cross::glsl::Target>::parse(&module)?;

                    // Target the right GLSL version
                    ast.set_compiler_options(&spirv_cross::glsl::CompilerOptions {
                        version,
                        ..Default::default()
                    })
                    .unwrap();

                    ast.compile()?
                };

                Ok(quote! { #out })
            }
            _ => unreachable!(),
        }
    }

    fn write_rust_wrapper(
        &self,
        shader_tokens: proc_macro2::TokenStream,
    ) -> crate::Result<proc_macro2::TokenStream> {
        // Shader resource structure
        let struct_name = format_ident!("{}", self.shader_struct_name());
        let kind_constant_name = format_ident!("{}", self.kind.constant_name);
        let st: syn::Type = syn::parse_str(if self.output_type.is_source() {
            "::tinygl::wrappers::SourceShader"
        } else {
            "::tinygl::wrappers::BinaryShader"
        })
        .unwrap();

        let mut parts = Vec::new();

        parts.push(quote! {
            pub struct #struct_name {
                name: tinygl::gl::Shader,
            }

            impl #struct_name {
                pub fn build(gl: &::tinygl::Context) -> ::tinygl::Result<Self> {
                    Ok(Self {
                        name: <Self as #st>::build(gl, ::tinygl::gl::#kind_constant_name)?
                    })
                }
            }

            impl ::tinygl::wrappers::ShaderCommon for #struct_name {
                fn kind(&self) -> u32 {
                    ::tinygl::gl::#kind_constant_name
                }

                fn name(&self) -> ::tinygl::gl::Shader {
                    self.name
                }
            }

            impl ::tinygl::wrappers::GlDrop for #struct_name {
                unsafe fn drop(&mut self, gl: &::tinygl::Context) {
                    use ::tinygl::wrappers::ShaderCommon;
                    gl.delete_shader(self.name());
                }
            }
        });

        // Implement the right shader trait for the given output type
        if self.output_type.is_source() {
            parts.push(quote! {
                impl ::tinygl::wrappers::SourceShader<'static> for #struct_name {
                    fn get_source() -> &'static str {
                        #shader_tokens
                    }
                }
            });
        } else {
            parts.push(quote! {
                impl ::tinygl::wrappers::BinaryShader<'static> for #struct_name {
                    fn get_binary() -> &'static [u8] {
                        #shader_tokens
                    }
                }
            });
        }

        // Write struct for holding uniform locations
        let struct_name = format_ident!("{}", self.uniform_struct_name());
        let uniform_location_name: Vec<_> = self
            .uniforms
            .iter()
            .map(|u| format_ident!("{}", u.location_name()))
            .collect();

        parts.push(quote! {
            #[derive(Default)]
            pub struct #struct_name {
                #(#uniform_location_name: Option<::tinygl::gl::UniformLocation>,)*
            }
        });

        let mut methods = Vec::new();

        // Write constructor
        let uset = self
            .uniforms
            .iter()
            .map(|uniform| {
                let name = format_ident!("{}", uniform.location_name());

                if self.output_type.is_source() {
                    // Source shader: find uniform locations from variable names
                    let uniform_name = uniform.name.as_str();
                    quote! { #name: unsafe { gl.get_uniform_location(program, #uniform_name) } }
                } else {
                    // Binary shader: assume locations form reflection on SPIR-V
                    let location = uniform.location;
                    quote! { #name: Some(#location) }
                }
            })
            .collect::<Vec<_>>();

        methods.push(quote! {
            pub fn new(gl: &::tinygl::Context, program: ::tinygl::gl::Program) -> Self {
                Self {
                    #(#uset),*
                }
            }
        });

        // Write getter/setter methods
        methods.extend(self.uniforms.iter().flat_map(|uniform| {
            let mut res = Vec::new();
            let ty = uniform.ty.unwrap();
            let type_name: syn::Type = syn::parse_str(&ty.rust_value_type()).unwrap();

            if let Some(binding) = uniform.binding {
                let meth_ident = format_ident!("get_{}_binding", uniform.name.to_snake_case());

                res.push(quote! {
                    pub fn #meth_ident(&self) -> #type_name {
                        #binding
                    }
                });
            }

            let meth_ident = format_ident!("set_{}", uniform.name.to_snake_case());
            let location = format_ident!("{}", uniform.location_name());
            let program_uniform = format_ident!("program_uniform{}", ty.uniform_method_name());
            let mut meth_args = vec![
                quote! { value: #type_name }
            ];

            if let Some(extra) = ty.uniform_method_extra_args_with_ty() {
                meth_args.insert(0, extra);
            }

            let mut call_args = Vec::new();

            if let Some(count) = ty.uniform_count_arg() {
                call_args.push(quote! { #count });
            }

            if let Some(extra) = ty.uniform_method_extra_args_no_ty() {
                call_args.push(extra);
            }

            call_args.push(ty.uniform_value(&format_ident!("value")));

            res.push(quote! {
                pub fn #meth_ident(&self, gl: &::tinygl::Context, program: ::tinygl::gl::ProgramName, #(#meth_args),*) {
                    if let Some(location) = self.#location {
                        unsafe {
                            gl.#program_uniform(program, location, #(#call_args),*);
                        }
                    }
                }
            });

            res
        }));

        let struct_name = self.uniform_struct_name();
        Ok(quote! {
            impl #struct_name {
                #(#methods)*
            }
        })
    }
}

impl WrappedItem for WrappedShader {
    fn generate(&self) -> crate::Result<proc_macro2::TokenStream> {
        self.write_rust_wrapper(self.write_shader()?)
    }
}
