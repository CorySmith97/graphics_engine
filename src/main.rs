#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod math;
mod shader;
use shader::*;
mod debug;
pub use debug::*;
mod mesh;
pub use mesh::*;

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
    event: sapp::Event,
    rx: f32,
    ry: f32,
    pos: Vec<m::Vec3>,
    path: &'a str,
    camera: Camera,
    mouse_toggle: bool,
}

struct Camera {
    position: m::Vec3,
    target: m::Vec3,
    up: m::Vec3,
    camera_front: m::Vec3,
    yaw: f32,
    pitch: f32,
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
    pip: sg::Pipeline::new(),
    bind: sg::Bindings::new(),
    event: sapp::Event::new(),
    rx: 0.,
    ry: 0.,
    pos: Vec::new(),
    path: "src/assets/awesomeface.png",
    camera: Camera {
        position: m::vec3(0., 0., 3.0),
        target: m::vec3(0., 0., 0.),
        up: m::vec3(0., 1., 0.),
        camera_front: m::vec3(0., 0., -1.),
        yaw: -90.,
        pitch: 0.,
    },
    mouse_toggle: false,
};

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

    let cube = Mesh::new_cube();

    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(cube.verticies),
        _type: sg::BufferType::Vertexbuffer,
        ..Default::default()
    });

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(cube.indices),
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
    let state = unsafe { &mut STATE };

    match event._type {
        sapp::EventType::KeyDown => {
            match event.key_code {
                sapp::Keycode::M => println!("TEST"),
                sapp::Keycode::W => {
                    state.camera.position = m::add_vec3(
                        state.camera.position,
                        m::scale_vec3(
                            6. * sapp::frame_duration() as f32,
                            state.camera.camera_front,
                        ),
                    );
                }
                sapp::Keycode::Comma => {
                    sapp::show_mouse(sapp::mouse_shown());
                }

                _ => (),
            }
            if event.key_code == sapp::Keycode::S {
                state.camera.position = m::sub_vec3(
                    state.camera.position,
                    m::scale_vec3(
                        6. * sapp::frame_duration() as f32,
                        state.camera.camera_front,
                    ),
                );
            }
            if event.key_code == sapp::Keycode::A {
                state.camera.position = m::sub_vec3(
                    state.camera.position,
                    m::scale_vec3(
                        6. * sapp::frame_duration() as f32,
                        m::norm_vec3(m::cross_vec3(state.camera.camera_front, state.camera.up)),
                    ),
                );
            }
            if event.key_code == sapp::Keycode::D {
                state.camera.position = m::add_vec3(
                    state.camera.position,
                    m::scale_vec3(
                        6. * sapp::frame_duration() as f32,
                        m::norm_vec3(m::cross_vec3(state.camera.camera_front, state.camera.up)),
                    ),
                );
            }
        }

        sapp::EventType::MouseMove => {
            let sensitivity = 0.1;
            let x_offset = event.mouse_dx * sensitivity;
            let y_offset = event.mouse_dy * sensitivity;

            state.camera.yaw += x_offset;
            state.camera.pitch -= y_offset;

            if state.camera.pitch > 89.0 {
                state.camera.pitch = 89.0
            };

            if state.camera.pitch < -89.0 {
                state.camera.pitch = -89.0
            };
            let yaw_rads = f32::to_radians(state.camera.yaw);
            // println!("Degree: {}, Rads: {}", state.camera.yaw, yaw_rads);

            let front = m::vec3(
                f32::cos(f32::to_radians(state.camera.yaw))
                    * f32::cos(f32::to_radians(state.camera.pitch)),
                f32::sin(f32::to_radians(state.camera.pitch)),
                f32::sin(f32::to_radians(state.camera.yaw))
                    * f32::cos(f32::to_radians(state.camera.pitch)),
            );

            state.camera.camera_front = m::norm_vec3(front);
            // println!("Camera Front {:?}", state.camera.camera_front);
        }

        _ => (),
    }
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
    println!("Frame Count: {:?}", sapp::frame_count());
    println!("Cam Pos: {:?}", state.camera.position);

    let t = (sapp::frame_duration()) as f32;

    state.rx += t * 30.;
    state.ry += t * 50.;

    let proj = m::persp_mat4(90.0, sapp::widthf() / sapp::heightf(), 0.01, 100.0);

    let view = m::lookat_mat4(
        state.camera.position,
        m::add_vec3(state.camera.position, state.camera.camera_front),
        m::vec3(0., 1., 0.),
    );

    //println!("View: {:?}", view[2]);

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
    let state = unsafe { &mut STATE };

    let vec = m::vec3(1.0, 0.0, 0.0);
    let model = m::translate_mat4(state.pos[(i - 1) as usize]);

    m::mul_mat4(view_proj, model)
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
        swap_interval: 1 / 120,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
