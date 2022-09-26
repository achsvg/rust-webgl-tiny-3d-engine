use nalgebra::{matrix, Matrix4};

use super::quaternion::Quaternion;

/// https://jsantell.com/3d-projection/#perspective-projection
pub fn from_frustrum(
    near: f32,
    far: f32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> Matrix4<f32> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    matrix![
        2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0;
        0.0, (2.0 * near) / (top - bottom), (top + bottom) / (top - bottom), 0.0;
        0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far);
        0.0, 0.0, -1.0, 0.0
    ]
}

pub fn from_fov_and_aspect(
    near: f32,
    far: f32,
    fov: f32,
    aspect: f32,
) -> Matrix4<f32> {
    let e = 1.0 / (-fov / 2.0).tan();
    #[cfg_attr(rustfmt, rustfmt_skip)]
    matrix![
        e / aspect, 0.0, 0.0, 0.0;
        0.0, e, 0.0, 0.0;
        0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far);
        0.0, 0.0, -1.0, 0.0
    ]
}

pub fn from_quaternion(q: &Quaternion) -> Matrix4<f32> {
    let xx = q.x * q.x;
    let yy = q.y * q.y;
    let zz = q.z * q.z;
    let xy = q.x * q.y;
    let zw = q.z * q.w;
    let xz = q.x * q.z;
    let yw = q.y * q.w;
    let yz = q.y * q.z;
    let xw = q.x * q.w;

    matrix![
        // row 1
        1.0 - 2.0 * yy - 2.0 * zz,
        2.0 * xy - 2.0 * zw,
        2.0 * xz + 2.0 * yw,
        0.0;
        // row 2
        2.0 * xy + 2.0 * zw,
        1.0 - 2.0 * xx - 2.0 * zz,
        2.0 * yz - 2.0 * xw,
        0.0;
        // row 3
        2.0 * xz - 2.0 * yw,
        2.0 * yz + 2.0 * xw,
        1.0 - 2.0 * xx - 2.0 * yy,
        0.0;
        // row 4
        0.0,
        0.0,
        0.0,
        1.0;
    ]
}

// TODO: Check if there is a more "rustic" way of converting
pub fn mat_to_col_array(mat: &Matrix4<f32>) -> [f32; 16] {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        mat.m11, mat.m21, mat.m31, mat.m41, 
        mat.m12, mat.m22, mat.m32, mat.m42, 
        mat.m13, mat.m23, mat.m33, mat.m43, 
        mat.m14, mat.m24, mat.m34, mat.m44, 
    ]
}

pub fn mat_to_row_array(mat: &Matrix4<f32>) -> [f32; 16] {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        mat.m11, mat.m12, mat.m13, mat.m14, 
        mat.m21, mat.m22, mat.m23, mat.m24, 
        mat.m31, mat.m32, mat.m33, mat.m34, 
        mat.m41, mat.m42, mat.m43, mat.m44, 
    ]
}
