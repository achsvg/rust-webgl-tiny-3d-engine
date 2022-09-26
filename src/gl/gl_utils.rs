use std::collections::HashMap;

use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation,
};

static DEFAULT_VERTEX_SHADER: &str = r##"#version 300 es
in vec3 position;
in vec4 color;
out vec4 vColor;

uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
    vec4 worldPosition = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
    gl_Position = worldPosition;
    vColor = color;
}
"##;

static DEFAULT_FRAGMENT_SHADER: &str = r##"#version 300 es
precision highp float;
in vec4 vColor;
out vec4 outColor;

void main() {
    outColor = vColor;
    //outColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"##;

pub struct Program {
    pub gl_program: WebGlProgram,
    pub vert_shader: WebGlShader,
    pub frag_shader: WebGlShader,
    pub uniform_locations: HashMap<String, WebGlUniformLocation>,
}

pub fn create_default_program(
    context: &WebGl2RenderingContext,
) -> Result<Program, String> {
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        DEFAULT_VERTEX_SHADER,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        DEFAULT_FRAGMENT_SHADER,
    )?;

    let program = link_program(context, &vert_shader, &frag_shader)?;

    let mut uniform_locations = HashMap::new();
    uniform_locations.insert(
        String::from("modelMatrix"),
        context
            .get_uniform_location(&program, "modelMatrix")
            .unwrap(),
    );

    uniform_locations.insert(
        String::from("viewMatrix"),
        context
            .get_uniform_location(&program, "viewMatrix")
            .unwrap(),
    );

    uniform_locations.insert(
        String::from("projectionMatrix"),
        context
            .get_uniform_location(&program, "projectionMatrix")
            .unwrap(),
    );

    Ok(Program {
        gl_program: program,
        vert_shader,
        frag_shader,
        uniform_locations,
    })
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let gl_program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&gl_program, vert_shader);
    context.attach_shader(&gl_program, frag_shader);
    context.link_program(&gl_program);

    if context
        .get_program_parameter(&gl_program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(gl_program)
    } else {
        Err(context
            .get_program_info_log(&gl_program)
            .unwrap_or_else(|| {
                String::from("Unknown error creating program object")
            }))
    }
}

pub fn get_context() -> Result<WebGl2RenderingContext, Object> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
}
