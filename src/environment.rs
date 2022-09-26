use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::gl::gl_utils;
use crate::gl::gl_utils::Program;
use crate::renderer::Renderer;
use crate::shapes::cuboid::Cuboid;
use crate::types::Color;
use crate::types::Vec3;
use crate::Environment;

pub struct GlEnvironment {
    pub context: WebGl2RenderingContext,
    pub program: Program,
}

static RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

static GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

static BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};

#[wasm_bindgen]
impl Environment {
    pub fn new(renderer: Renderer) -> Result<Environment, JsValue> {
        console_error_panic_hook::set_once();
        let context = gl_utils::get_context()?;
        let program = gl_utils::create_default_program(&context)?;

        context.use_program(Some(&program.gl_program));
        context.enable(WebGl2RenderingContext::CULL_FACE);

        let gl_environment = GlEnvironment { context, program };

        let mut cuboid = Cuboid::new(
            &gl_environment,
            1.0,
            1.0,
            1.0,
            &[RED, BLUE, GREEN, RED],
        )?;
        cuboid.shape.transform.translate(0.0, 0.0, -5.0);

        Ok(Environment {
            renderer,
            gl_environment,
            shapes: vec![cuboid.shape],
            tick_count: 0,
        })
    }

    pub fn tick(&mut self) {
        self.shapes[0]
            .transform
            .rotate(Vec3::new(0.0, 1.0, 0.0), 1.0);

        // TODO: prepare for render asynchronously.
        for shape in &mut self.shapes {
            shape.prepare_for_render(&self.gl_environment.context);
        }

        self.renderer.render(&self.gl_environment, &self.shapes);

        self.tick_count += 1;
    }
}
