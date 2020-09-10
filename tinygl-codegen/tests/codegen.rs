#[cfg(all(feature = "spirv", feature = "backend-shaderc"))]
#[test]
fn test_codegen() -> tinygl_compiler::Result<()> {
    use tinygl_codegen::compiler::WrappedItem;
    use tinygl_compiler::{model::*, reflect, Compiler, ShaderKind};

    // Initialize shaderc compiler
    let mut compiler = Compiler::new(true, None)?.with_shaderc();

    // Initialize reflection backend
    let reflector = reflect::SpirVBackend::new();

    // Load shader objects, turn them into SPIR-V, reflect uniforms
    let vert = GlslObject::from_str(include_str!("../../shaders/quad.vert"), ShaderKind::Vertex)?
        .compile(&mut compiler)?
        .reflect_spirv(&reflector)?;
    let frag = GlslObject::from_str(include_str!("../../shaders/uv.frag"), ShaderKind::Fragment)?
        .compile(&mut compiler)?
        .reflect_spirv(&reflector)?;

    // Check reflection result
    assert_eq!(vert.uniforms().len(), 0);
    assert_eq!(frag.uniforms().len(), 1);

    assert!(compiler.wrap_shader(vert, true)?.generate().is_ok());

    Ok(())
}
