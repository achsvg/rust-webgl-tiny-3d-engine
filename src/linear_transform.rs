use nalgebra::Matrix4;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    math::{matrix::from_quaternion, quaternion::Quaternion},
    types::Vec3,
};

#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct LinearTransform {
    translation: Vec3,
    rotation: Quaternion,
    scale: f32,
    matrix: Matrix4<f32>,
    needs_update: bool,
}

impl LinearTransform {
    pub fn new() -> Self {
        LinearTransform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quaternion::identity(),
            scale: 1.0,
            matrix: Matrix4::identity(),
            needs_update: false,
        }
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.rotation *= &Quaternion::from_axis_angle(axis, angle);
        self.rotation.normalize();
        self.needs_update = true;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.translation.x += x;
        self.translation.y += y;
        self.translation.z += z;
        self.needs_update = true;
    }

    pub fn scale(&mut self, value: f32) {
        self.scale *= value;
        self.needs_update = true;
    }

    pub fn get_matrix(&self) -> &Matrix4<f32> {
        &self.matrix
    }

    pub fn update_matrix(&mut self) {
        if self.needs_update {
            // TODO: Optimize this.
            let rot_mat = from_quaternion(&self.rotation);
            let scale_mat = Matrix4::identity() * self.scale;
            let mut trans_mat = Matrix4::identity();
            trans_mat[(3, 0)] = self.translation.x;
            trans_mat[(3, 1)] = self.translation.y;
            trans_mat[(3, 2)] = self.translation.z;
            self.matrix = rot_mat * scale_mat * trans_mat;
            self.needs_update = false;
        }
    }
}
