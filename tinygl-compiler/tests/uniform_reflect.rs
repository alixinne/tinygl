use tinygl_compiler::{model::GlslObject, reflect, Compiler, Result, ShaderKind};

fn find<'p>(
    program: &'p tinygl_compiler::WrappedProgram,
    uniform_name: &str,
) -> Option<&'p tinygl_compiler::reflect::FoundUniform> {
    program
        .shaders()
        .flat_map(|shader| shader.uniforms().iter())
        .find(|uniform| uniform.name == uniform_name)
}

#[test]
#[allow(unused_variables)]
fn uniform_reflect() -> Result<()> {
    let mut compiler = Compiler::new(true, None)?.with_shaderc();

    let backend = reflect::SpirVBackend::new();

    // Prepare shaders by compiling to SPIR-V and reflecting
    let vert_shader =
        GlslObject::from_str(include_str!("../../shaders/quad.vert"), ShaderKind::Vertex)?
            .compile(&mut compiler)?
            .reflect_spirv(&backend)?;

    let frag_shader = GlslObject::from_str(
        include_str!("../../shaders/uniform_reflect.frag"),
        ShaderKind::Fragment,
    )?
    .compile(&mut compiler)?
    .reflect_spirv(&backend)?;

    // Wrap the shaders
    let vert_shader = compiler.wrap_shader(vert_shader, true)?;

    let frag_shader = compiler.wrap_shader(frag_shader, true)?;

    // Wrap program
    let program = compiler.wrap_program(&[&vert_shader, &frag_shader], "program")?;

    for s in program.shaders() {
        eprintln!("{:?}", s.uniforms());
    }

    let uniform_names = [
        "testFloat",
        "testFloatVec2",
        "testFloatVec3",
        "testFloatVec4",
        "testDouble",
        "testDoubleVec2",
        "testDoubleVec3",
        "testDoubleVec4",
        "testInt",
        "testIntVec2",
        "testIntVec3",
        "testIntVec4",
        "testUnsignedInt",
        "testUnsignedIntVec2",
        "testUnsignedIntVec3",
        "testUnsignedIntVec4",
        "testBool",
        "testBoolVec2",
        "testBoolVec3",
        "testBoolVec4",
        "testFloatMat2",
        "testFloatMat3",
        "testFloatMat4",
        // Disabled: rectangular matrices not supported
        //"testFloatMat2x3",
        //"testFloatMat2x4",
        //"testFloatMat3x2",
        //"testFloatMat3x4",
        //"testFloatMat4x2",
        //"testFloatMat4x3",
        "testDoubleMat2",
        "testDoubleMat3",
        "testDoubleMat4",
        // Disabled: rectangular matrices not supported
        //"testDoubleMat2x3",
        //"testDoubleMat2x4",
        //"testDoubleMat3x2",
        //"testDoubleMat3x4",
        //"testDoubleMat4x2",
        //"testDoubleMat4x3",
        "testSampler1D",
        "testSampler2D",
        "testSampler3D",
        "testSamplerCube",
        "testSampler1DShadow",
        "testSampler2DShadow",
        "testSampler1DArray",
        "testSampler2DArray",
        "testSampler1DArrayShadow",
        "testSampler2DArrayShadow",
        "testSampler2DMS",
        "testSampler2DMSArray",
        "testSamplerCubeShadow",
        "testSamplerBuffer",
        "testSampler2DRect",
        "testSampler2DRectShadow",
        "testISampler1D",
        "testISampler2D",
        "testISampler3D",
        "testISamplerCube",
        "testISampler1DArray",
        "testISampler2DArray",
        "testISampler2DMS",
        "testISampler2DMSArray",
        "testISamplerBuffer",
        "testISampler2DRect",
        "testUSampler1D",
        "testUSampler2D",
        "testUSampler3D",
        "testUSamplerCube",
        "testUSampler1DArray",
        "testUSampler2DArray",
        "testUSampler2DMS",
        "testUSampler2DMSArray",
        "testUSamplerBuffer",
        "testUSampler2DRect",
    ];

    for uniform_name in uniform_names.iter() {
        assert!(
            find(&program, uniform_name).is_some(),
            "missing uniform: {}",
            uniform_name
        );
    }

    let set = compiler
        .wrap_uniforms(&[&program], "global")
        .expect("failed to wrap uniforms");

    tinygl_compiler::codegen::write(
        std::env::temp_dir().join("shaders.rs"),
        &[&vert_shader, &frag_shader, &program, &set],
    )?;

    Ok(())
}
