use web_sys::WebGl2RenderingContext;

use crate::{
    environment::GlEnvironment,
    gl::gl_shape::{GlShape, ShapeProps},
    types::Color,
};

pub struct Cuboid {
    pub shape: GlShape,
}

impl Cuboid {
    pub fn new(
        gl_environment: &GlEnvironment,
        width: f32,
        height: f32,
        depth: f32,
        colors: &[Color; 4],
    ) -> Result<Cuboid, String> {
        let hwidth = width / 2.0;
        let hheight = height / 2.0;
        let hdepth = depth / 2.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let positions = vec![
            hwidth, hheight, hdepth, // Front-top-right 0
            -hwidth, hheight, hdepth, // Front-top-left 2
            hwidth, hheight, -hdepth, // Back-top-right 3
            -hwidth, hheight, -hdepth, // Back-top-left 4
            hwidth, -hheight, hdepth, // Front-bottom-right 5
            -hwidth, -hheight, hdepth, // Front-bottom-left 6
            -hwidth, -hheight, -hdepth, // Back-bottom-left 7
            hwidth, -hheight, -hdepth, // Back-bottom-right 8
        ];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let colors = vec![
            colors[0].r, colors[0].g, colors[0].b, colors[0].a, // Front-top-left
            colors[1].r, colors[1].g, colors[1].b, colors[1].a, // Front-top-right
            colors[2].r, colors[2].g, colors[2].b, colors[2].a, // Front-bottom-left
            colors[3].r, colors[3].g, colors[3].b, colors[3].a, // Front-bottom-right
            colors[0].r, colors[0].g, colors[0].b, colors[0].a, // Back-top-right
            colors[1].r, colors[1].g, colors[1].b, colors[1].a, // Back-top-left
            colors[2].r, colors[2].g, colors[2].b, colors[2].a, // Back-bottom-left
            colors[3].r, colors[3].g, colors[3].b, colors[3].a, // Back-bottom-right
        ];

        let shape = GlShape::new(ShapeProps {
            env: &gl_environment,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
            vertices_count: 8,
            positions,
            colors,
            indices: Some(vec![3, 2, 6, 7, 4, 2, 0, 3, 1, 6, 5, 4, 1, 0]),
        })?;

        Ok(Cuboid { shape })
    }
}
