use std::ops;

use crate::types::Vec3;

use super::utils;

#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle_in_degree: f32) -> Self {
        let angle = utils::degree_to_radian(angle_in_degree);
        let x = axis.x * f32::sin(angle / 2.0);
        let y = axis.y * f32::sin(angle / 2.0);
        let z = axis.z * f32::sin(angle / 2.0);
        let w = f32::cos(angle / 2.0);
        let mut q = Self { x, y, z, w };
        q.normalize();
        q
    }

    pub fn normalize(&mut self) {
        let n = f32::sqrt(
            self.x * self.x
                + self.y * self.y
                + self.z * self.z
                + self.w * self.w,
        );

        self.x /= n;
        self.y /= n;
        self.z /= n;
        self.w /= n;
    }

    // pub fn rotate(&self, vec: &[f32; 3]) -> [f32; 3] {
    //     self.matrix.multiply(vec)
    // }
}

fn multiply(q1: &Quaternion, q2: &Quaternion) -> Quaternion {
    Quaternion {
        x: q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
        y: q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
        z: q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
        w: q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
    }
}

impl ops::Mul<&Quaternion> for Quaternion {
    type Output = Self;
    fn mul(self, _rhs: &Self) -> Self {
        multiply(&self, _rhs)
    }
}

impl ops::MulAssign<&Quaternion> for Quaternion {
    fn mul_assign(&mut self, _rhs: &Self) {
        let q = multiply(&self, _rhs);
        *self = q;
    }
}
