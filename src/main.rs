mod math;
mod shader;
use shader::*;
mod debug;
pub use debug::*;
use math as m;
use sokol::{
    app::{self as sapp, Keycode},
    debugtext as stdx,
    gfx::{self as sg, Image},
    glue as sglue, log as slog,
};

use image::*;

use std::path::Path;
use std::time::{Duration, Instant};

use crate::shader::SLOT_SMP;

const FONT_KC853: usize = 0;

struct State<'a> {
    pass_action: sg::PassAction,
    pip: sg::Pipeline,
    bind: sg::Bindings,
    rx: f32,
    ry: f32,
    pos: Vec<m::Vec3>,
    path: &'a str,
}

struct Camera {
    position: m::Vec3,
    target: m::Vec3,
    up: m::Vec3,
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
    pip: sg::Pipeline::new(),
    bind: sg::Bindings::new(),
    rx: 0.,
    ry: 0.,
    pos: Vec::new(),
    path: "../assets/awesomeface.png",
};

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub color: u32,
    pub u: u16,
    pub v: u16,
}

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        logger: sg::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut text_desc = stdx::Desc::new();
    text_desc.fonts[FONT_KC853] = stdx::font_kc853();
    stdx::setup(&text_desc);

    state.bind.fs.images[SLOT_TEX] = sg::alloc_image();

    state.bind.fs.samplers[SLOT_SMP] = sg::make_sampler(&sg::SamplerDesc {
        ..Default::default()
    });

    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        // pos                                color              uvs
        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFF0000FF, u:     0, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF0000, u:     0, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFFFF0000, u: 32767, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFFFF0000, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF0000, u:     0, v: 32767 },

        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF007F, u:     0, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFFFF007F, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFFFF007F, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF007F, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v:     0 },
        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v: 32767 },

        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v: 32767 },
    ];

    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0, 1, 2,  0, 2, 3,
        6, 5, 4,  7, 6, 4,
        8, 9, 10,  8, 10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20,
    ];

    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(VERTICES),
        _type: sg::BufferType::Vertexbuffer,
        ..Default::default()
    });

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(INDICES),
        _type: sg::BufferType::Indexbuffer,
        ..Default::default()
    });
    let image = image::open(state.path).expect("Failed to open");
    let pixels = convert_to_u32_array(&image);
    let mut image_desc = sg::ImageDesc {
        width: WIDTH as i32,
        height: HEIGHT as i32,
        ..Default::default()
    };
    image_desc.data.subimage[0][0] = sg::slice_as_range(&pixels);
    state.bind.fs.images[SLOT_TEX] = sg::make_image(&&image_desc);

    #[rustfmt::skip]
    let pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::shader_shader_desc(sg::query_backend())),
        layout: sg::VertexLayoutState {
            attrs: {
                let mut attrs = [sg::VertexAttrState::new(); sg::MAX_VERTEX_ATTRIBUTES];

                attrs[shader::ATTR_VS_POS] = sg::VertexAttrState { format: sg::VertexFormat::Float3, ..Default::default() };
                attrs[shader::ATTR_VS_COLOR] = sg::VertexAttrState { format: sg::VertexFormat::Ubyte4, ..Default::default() };
                attrs[shader::ATTR_VS_TEXCOORDS0] = sg::VertexAttrState { format: sg::VertexFormat::Short2n, ..Default::default()};

                attrs
            },
            ..Default::default()

        },
        index_type: sg::IndexType::Uint16,
        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,

            ..Default::default()
        },
        ..Default::default()
    });

    state.pip = pip;

    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color {
            r: 0.25,
            g: 0.5,
            b: 0.75,
            a: 1.0,
        },
        ..Default::default()
    };

    let pos1 = m::vec3(0., 0., 0.);

    state.pos.push(pos1);
    state.pos.push(m::vec3(1., 0., 0.1));
    state.pos.push(m::vec3(-1.3, 1., 0.8));
    state.pos.push(m::vec3(0.3, -2., 1.));
    state.pos.push(m::vec3(0., 2., 1.));
}

extern "C" fn event(event: *const sapp::Event) {
    let event = unsafe { &*event };

    if event._type == sapp::EventType::KeyDown {
        if event.key_code == sapp::Keycode::A {
            println!("!");
        }
    }

    if event._type == sapp::EventType::MouseMove {}
}
const HEIGHT: usize = 512;
const WIDTH: usize = 512;

fn convert_to_u32_array(img: &DynamicImage) -> [u32; WIDTH * HEIGHT] {
    // Create a fixed-size array to store the pixel data
    let mut pixel_data: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    // Iterate over each pixel in the image
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Get the RGBA color of the pixel
            let rgba_pixel = img.get_pixel(x as u32, y as u32).0;

            // Convert the RGBA color to a u32 value
            let u32_pixel =
                u32::from_ne_bytes([rgba_pixel[0], rgba_pixel[1], rgba_pixel[2], rgba_pixel[3]]);

            // Assign the u32 value to the appropriate index in the array
            pixel_data[y * WIDTH + x] = u32_pixel;
        }
    }

    // Return the resulting fixed-size array
    pixel_data
}

extern "C" fn frame() {
    let start = Instant::now();
    let state = unsafe { &mut STATE };

    let t = (sapp::frame_duration()) as f32;

    state.rx += t * 30.;
    state.ry += t * 50.;

    let proj = m::persp_mat4(45.0, sapp::widthf() / sapp::heightf(), 0.01, 100.0);

    let view = m::lookat_mat4(
        m::vec3(0., 2., 6.),
        m::vec3(0., 0., 0.),
        m::vec3(0., 1., 0.),
    );

    let view_proj = m::mul_mat4(proj, view);
    let (width, height) = (sapp::width(), sapp::height());

    sg::begin_default_pass(&state.pass_action, width, height);
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    let vs_params = shader::VsParams {
        mvp: compute_mvp(state.rx, state.ry, 1, view_proj),
    };
    sg::apply_uniforms(
        sg::ShaderStage::Vs,
        shader::SLOT_VS_PARAMS,
        &sg::value_as_range(&vs_params),
    );
    sg::draw(0, 36, 1);
    debug_stats(start);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    stdx::shutdown();
    sg::shutdown()
}

pub fn compute_mvp(rx: f32, ry: f32, i: i32, view_proj: m::Mat4) -> [[f32; 4]; 4] {
    let mut state = unsafe { &mut STATE };

    let angle = 20.0 * rx * (i as f32 - 1.0);
    let vec = m::vec3(1.0, 0.0, 0.0);

    let rotation = m::rotate_mat4(rx, vec);
    let rym = m::rotate_mat4(ry, m::vec3(0., 1.0, 0.));
    let model = m::translate_mat4(state.pos[(i - 1) as usize]);

    let rm = m::mul_mat4(rotation, rym);
    m::mul_mat4(view_proj, rm)
}

fn main() {
    let window_title = b"Engine\0".as_ptr() as _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        event_cb: Some(event),
        window_title,
        width: 800,
        height: 600,
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
