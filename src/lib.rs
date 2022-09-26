#![allow(unused_variables)]

use environment::GlEnvironment;
use gl::gl_shape::GlShape;
use renderer::Renderer;
use wasm_bindgen::prelude::wasm_bindgen;

#[macro_use]
pub mod utils;

pub mod camera;
mod environment;
mod gl {
    pub mod gl_shape;
    pub mod gl_utils;
}
mod math {
    pub mod matrix;
    pub mod quaternion;
    pub mod utils;
}
pub mod linear_transform;
mod renderer;
mod shapes {
    pub mod cuboid;
    pub mod triangle;
}
pub mod types;

#[wasm_bindgen]
pub struct Environment {
    gl_environment: GlEnvironment,
    shapes: Vec<GlShape>,
    tick_count: u32,
    renderer: Renderer,
}
