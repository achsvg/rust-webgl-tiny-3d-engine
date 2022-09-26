use std::convert::TryInto;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;

use crate::environment::GlEnvironment;
use crate::linear_transform::LinearTransform;
use crate::utils::Buffer;

static POSITION_DATA_SIZE: usize = 3;
static COLOR_DATA_SIZE: usize = 4;
static VERTEX_DATA_SIZE: usize = POSITION_DATA_SIZE + COLOR_DATA_SIZE;
static BYTES_PER_FLOAT: usize = 4;

pub struct GlShape {
    pub vao: WebGlVertexArrayObject,
    pub gl_array_buffer: WebGlBuffer,
    pub gl_element_buffer: Option<WebGlBuffer>,
    pub mode: u32,
    pub vertices_count: usize,
    pub array_buffer: Vec<f32>,
    pub element_array: Option<Vec<u32>>,
    pub transform: LinearTransform,

    positions: Buffer<Vec<f32>>,
    colors: Buffer<Vec<f32>>,
}

pub struct ShapeProps<'a> {
    pub env: &'a GlEnvironment,
    pub mode: u32,
    pub vertices_count: usize,
    pub positions: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Option<Vec<u32>>, // if using element array buffer
}

impl GlShape {
    fn pack_attributes(&mut self) {
        if !self.positions.is_dirty && !self.colors.is_dirty {
            return;
        }
        #[cfg_attr(rustfmt, rustfmt_skip)]
        for i in 0..self.vertices_count {
            let buf_pos_start: usize = i * VERTEX_DATA_SIZE;
            let buf_pos_end: usize = buf_pos_start + POSITION_DATA_SIZE;
            let pos_start: usize = i * POSITION_DATA_SIZE;
            let pos_end: usize = pos_start + POSITION_DATA_SIZE;
            let buf_color_start: usize = i * VERTEX_DATA_SIZE + POSITION_DATA_SIZE;
            let buf_color_end: usize = buf_color_start + COLOR_DATA_SIZE;
            let color_start: usize = i * COLOR_DATA_SIZE;
            let color_end: usize = color_start + COLOR_DATA_SIZE;
            if self.positions.is_dirty {
                self.array_buffer[buf_pos_start..buf_pos_end].copy_from_slice(&self.positions.buffer[pos_start..pos_end]);
            }
            if self.colors.is_dirty {
                self.array_buffer[buf_color_start..buf_color_end].copy_from_slice(&self.colors.buffer[color_start..color_end]);
            }
        }
        self.positions.is_dirty = false;
        self.colors.is_dirty = false
    }

    pub fn buffer_needs_update(&self) -> bool {
        self.positions.is_dirty || self.colors.is_dirty
    }

    pub fn prepare_for_render(&mut self, context: &WebGl2RenderingContext) {
        self.transform.update_matrix();
        if self.buffer_needs_update() {
            self.pack_attributes();
            with_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                context,
                &self.gl_array_buffer,
                |_buffer| {
                    update_buffer(context, _buffer, &self.array_buffer);
                },
            );
        }
    }

    pub fn new(props: ShapeProps) -> Result<GlShape, String> {
        assert!(
            props.positions.len() == POSITION_DATA_SIZE * props.vertices_count,
            "positions size = {}, expected {}",
            props.positions.len(),
            POSITION_DATA_SIZE * props.vertices_count
        );

        assert!(
            props.colors.len() == COLOR_DATA_SIZE * props.vertices_count,
            "colors size = {}, expected {}",
            props.colors.len(),
            COLOR_DATA_SIZE * props.vertices_count
        );

        let vao = props
            .env
            .context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        let gl_array_buffer = create_buffer(&props.env)?;

        let gl_element_buffer = props
            .indices
            .as_ref()
            .and(Some(create_buffer(&props.env).unwrap()));

        with_vao(&props.env.context, &vao, |_| {
            if let Some(element_buffer) = &gl_element_buffer {
                props.env.context.bind_buffer(
                    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                    Some(&element_buffer),
                );
            }
            with_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &props.env.context,
                &gl_array_buffer,
                |_buffer: &WebGlBuffer| {
                    init_attributes(&props.env, _buffer);
                },
            )
        });

        if let Some(element_buffer) = &gl_element_buffer {
            with_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &props.env.context,
                &element_buffer,
                |_buffer| {
                    update_element_buffer(
                        &props.env.context,
                        _buffer,
                        props.indices.as_ref().unwrap(),
                    );
                },
            );
        }

        let buffer_size: usize = (VERTEX_DATA_SIZE * props.vertices_count)
            .try_into()
            .unwrap();

        Ok(GlShape {
            vao,
            gl_array_buffer,
            gl_element_buffer: gl_element_buffer,
            mode: props.mode,
            vertices_count: props.vertices_count,
            array_buffer: vec![0.0; buffer_size],
            element_array: props.indices,
            positions: Buffer {
                buffer: props.positions,
                is_dirty: true,
            },
            colors: Buffer {
                buffer: props.colors,
                is_dirty: true,
            },
            transform: LinearTransform::new(),
        })
    }
}

fn init_attributes(env: &GlEnvironment, buffer: &WebGlBuffer) {
    let stride = VERTEX_DATA_SIZE * BYTES_PER_FLOAT;
    configure_attribute(
        &env,
        &buffer,
        "position",
        POSITION_DATA_SIZE as i32,
        stride as i32,
        0,
    );
    configure_attribute(
        &env,
        &buffer,
        "color",
        COLOR_DATA_SIZE as i32,
        stride as i32,
        (POSITION_DATA_SIZE * BYTES_PER_FLOAT) as i32,
    );
}

fn create_buffer(env: &GlEnvironment) -> Result<WebGlBuffer, String> {
    let buffer = env
        .context
        .create_buffer()
        .ok_or("Failed to create buffer")?;
    Ok(buffer)
}

fn configure_attribute(
    env: &GlEnvironment,
    buffer: &WebGlBuffer,
    name: &str,
    size: i32,
    stride: i32,
    offset: i32,
) {
    let attribute_location = env
        .context
        .get_attrib_location(&env.program.gl_program, name);
    env.context.vertex_attrib_pointer_with_i32(
        attribute_location as u32,
        size,
        WebGl2RenderingContext::FLOAT,
        false,
        stride,
        offset,
    );
    env.context
        .enable_vertex_attrib_array(attribute_location as u32);
}

pub fn update_buffer(
    context: &WebGl2RenderingContext,
    buffer: &WebGlBuffer,
    data: &Vec<f32>,
) {
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(data);

        // TODO: use bufferSubData to avoid reallocating data store.
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );
    }
}

pub fn update_element_buffer(
    context: &WebGl2RenderingContext,
    buffer: &WebGlBuffer,
    data: &Vec<u32>,
) {
    unsafe {
        let positions_array_buf_view = js_sys::Uint32Array::view(data);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn with_buffer<F>(
    target: u32,
    context: &WebGl2RenderingContext,
    buffer: &WebGlBuffer,
    func: F,
) where
    F: Fn(&WebGlBuffer),
{
    context.bind_buffer(target, Some(buffer));
    func(&buffer);
    context.bind_buffer(target, None);
}

pub fn with_vao<F>(
    context: &WebGl2RenderingContext,
    vao: &WebGlVertexArrayObject,
    func: F,
) where
    F: Fn(&WebGlVertexArrayObject),
{
    context.bind_vertex_array(Some(&vao));
    func(&vao);
    context.bind_vertex_array(None);
}
