use web_sys::WebGl2RenderingContext;

use crate::{
    environment::GlEnvironment,
    gl::gl_shape::{GlShape, ShapeProps},
    types::{Color, Vec3},
};

pub struct Triangle {
    pub shape: GlShape,
}

impl Triangle {
    /// Points should be clockwise
    pub fn new(
        gl_environment: &GlEnvironment,
        vertices: &[Vec3; 3],
        colors: &[Color; 3],
    ) -> Result<Triangle, String> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let positions = vec![
            vertices[0].x, vertices[0].y, vertices[0].z,
            vertices[1].x, vertices[1].y, vertices[1].z,
            vertices[2].x, vertices[2].y, vertices[2].z,
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let colors = vec![
            colors[0].r, colors[0].g, colors[0].b, colors[0].a,
            colors[1].r, colors[1].g, colors[1].b, colors[1].a,
            colors[2].r, colors[2].g, colors[2].b, colors[2].a,
        ];

        let shape = GlShape::new(ShapeProps {
            env: &gl_environment,
            mode: WebGl2RenderingContext::TRIANGLES,
            vertices_count: 3,
            positions,
            colors,
            indices: None,
        })?;

        Ok(Triangle { shape })
    }
}
