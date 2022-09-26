extern crate nalgebra as na;

use na::Matrix4;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    linear_transform::LinearTransform, math::matrix::from_fov_and_aspect,
};

#[wasm_bindgen]
pub struct Camera {
    projection: Matrix4<f32>,
    pub transform: LinearTransform,
}

#[wasm_bindgen]
impl Camera {
    pub fn new(near: f32, far: f32, fov: f32, aspect: f32) -> Self {
        // let top = near * (fov / 2.0).tan();
        // let bottom = -top;
        // let right = aspect * top;
        // let left = -right;
        Self {
            transform: LinearTransform::new(),
            // projection: Matrix4::identity(),
            projection: from_fov_and_aspect(near, far, fov, aspect),
            // projection: from_frustrum(1.0, 100.0, -10.0, 10.0, 10.0, -10.0),
        }
    }
}

impl Camera {
    pub fn get_projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection
    }
}
