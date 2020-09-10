use tinygl_compiler::model::*;
use tinygl_compiler::{GlslVersion, Result, ShaderKind};

#[test]
fn test_glsl_from_string() -> Result<()> {
    let mut object =
        GlslObject::from_str(include_str!("../../shaders/quad.vert"), ShaderKind::Vertex)?;

    // No version without parsing
    assert_eq!(object.version(), None);

    // Parse the source
    object.parse()?;

    // Check that we parsed the version correctly
    assert_eq!(object.version(), Some(GlslVersion::V4_60));

    Ok(())
}

#[cfg(feature = "spirv")]
#[test]
fn test_spirv_from_bytes() -> Result<()> {
    let _object =
        SpirVObject::from_bytes(include_bytes!("../../shaders/quad.spv"), ShaderKind::Vertex)?;

    Ok(())
}

#[cfg(all(feature = "spirv", feature = "backend-shaderc"))]
#[test]
fn test_basic_program() -> Result<()> {
    use tinygl_compiler::{reflect, Compiler};

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

    Ok(())
}
