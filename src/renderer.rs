use std::convert::TryInto;

use nalgebra::Matrix4;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_timer::Instant;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use crate::{
    camera::Camera,
    environment::GlEnvironment,
    gl::gl_shape::{with_vao, GlShape},
    math::matrix::mat_to_col_array,
};

#[wasm_bindgen]
pub struct Renderer {
    camera: Camera,
    time_since_last_render: Option<Instant>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(camera: Camera) -> Renderer {
        Renderer {
            camera,
            time_since_last_render: None,
        }
    }
}

impl Renderer {
    pub fn render(&mut self, env: &GlEnvironment, shapes: &Vec<GlShape>) {
        let elapsed = self
            .time_since_last_render
            .unwrap_or(Instant::now())
            .elapsed()
            .as_secs();
        self.camera.transform.update_matrix();
        self.draw(env, &self.camera.transform.get_matrix(), shapes)
            .unwrap();
        self.time_since_last_render = Some(Instant::now());
    }

    fn draw(
        &self,
        env: &GlEnvironment,
        view_matrix: &Matrix4<f32>,
        shapes: &Vec<GlShape>,
    ) -> Result<(), String> {
        env.context.clear_color(0.0, 0.0, 0.0, 1.0);
        env.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        let model_matrix_loc =
            env.program.uniform_locations.get("modelMatrix").unwrap();

        set_uniform(
            env,
            "viewMatrix",
            &mat_to_col_array(&view_matrix.try_inverse().unwrap()),
        )
        .unwrap();

        set_uniform(
            env,
            "projectionMatrix",
            &mat_to_col_array(&self.camera.get_projection_matrix()),
        )
        .unwrap();

        for shape in shapes {
            draw_shape(&env.context, shape, model_matrix_loc);
        }
        Ok(())
    }
}

fn set_uniform(
    env: &GlEnvironment,
    name: &str,
    data: &[f32],
) -> Result<(), String> {
    if !env.program.uniform_locations.contains_key(name) {
        let error = format!("Could not find {} location.", name);
        return Err(error);
    }
    env.context.uniform_matrix4fv_with_f32_array(
        env.program.uniform_locations.get(name),
        true,
        data,
    );
    Ok(())
}

fn draw_shape(
    context: &WebGl2RenderingContext,
    shape: &GlShape,
    model_matrix_loc: &WebGlUniformLocation,
) {
    let transformation_matrix = shape.transform.get_matrix();

    context.uniform_matrix4fv_with_f32_array(
        Some(model_matrix_loc),
        true,
        &mat_to_col_array(&transformation_matrix),
    );

    with_vao(context, &shape.vao, |_| match &shape.element_array {
        Some(array) => context.draw_elements_with_i32(
            shape.mode,
            array.len().try_into().unwrap(),
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        ),
        None => context.draw_arrays(shape.mode, 0, shape.vertices_count as i32),
    });
}
