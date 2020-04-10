#[test]
#[allow(unused_variables)]
fn uniform_reflect() {
    let mut compiler = tinygl_compiler::CompilerBuilder::new()
        .build()
        .expect("failed to build compiler");

    let vert_shader = compiler
        .wrap_shader_source(
            include_str!("../../shaders/quad.vert"),
            tinygl_compiler::ShaderKind::Vertex,
        )
        .expect("failed to compile vertex shader")
        .into_id();

    let frag_shader = compiler
        .wrap_shader_source(
            include_str!("../../shaders/uniform_reflect.frag"),
            tinygl_compiler::ShaderKind::Fragment,
        )
        .expect("failed to compile fragment shader")
        .into_id();

    let program = compiler
        .wrap_program(&[&vert_shader, &frag_shader], "program")
        .expect("failed to wrap program")
        .into_id();

    let set = compiler
        .wrap_uniforms(&[&program], "global")
        .expect("failed to wrap uniforms");
}
