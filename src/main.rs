extern crate cgmath;
extern crate gl;
extern crate rand;
extern crate sdl2;

mod particle_system;
mod graphics;
mod camera;

use graphics::shader;
use particle_system::ParticleSystem;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Instant;
use std::os::raw::c_void;
use std::os::raw::c_char;
use camera::Camera;
use sdl2::keyboard::Scancode;


fn render(particle_system: &mut ParticleSystem, cam: &Camera) {
    particle_system.render(cam);

    unsafe {
        gl::Flush();
    }
}


#[no_mangle]
pub extern "system" fn debug_callback(
    _: gl::types::GLenum,
    err_type: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _: gl::types::GLsizei,
    message: *const c_char,
    _: *mut c_void,
) {
    unsafe {
        let err_text = std::ffi::CStr::from_ptr(message);
        println!(
            "Type: {:#x} ID: {:#x} Severity: {:#x}:\n  {:#?}",
            err_type,
            id,
            severity,
            err_text.to_str().unwrap()
        )
    }
}


fn handle_input(
    cam: &mut Camera,
    keyboard_state: &sdl2::keyboard::KeyboardState,
    dx: i32,
    dy: i32,
    dt: f32,
) {
    cam.angle_pitch -= dy as f32 / 25.0f32;
    cam.angle_yaw += dx as f32 / 25.0f32;

    let distance = 50.0f32 * dt;
    let mut speed_multiplier = 1.0f32;

    if keyboard_state.is_scancode_pressed(Scancode::LShift) {
        speed_multiplier *= 3.0;
    }

    if keyboard_state.is_scancode_pressed(Scancode::S) {
        cam.position.z -= distance * speed_multiplier;
    } else if keyboard_state.is_scancode_pressed(Scancode::W) {
        cam.position.z += distance * speed_multiplier;
    }

    cam.update_matrices();
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    let mut cam = Camera::new();
    let mut pause_dt = false;

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();

    gl_attr.set_context_version(4, 3);

    let window = video_subsystem
        .window("Rust SDL window", 1600, 900)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context();
    println!("Started with GL version: {:?}", gl_attr.context_version());

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        gl::DebugMessageCallback(debug_callback, std::ptr::null())
    };

    video_subsystem.gl_set_swap_interval(1);

    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe { println!("OpenGL version is {:?}", gl::GetString(gl::VERSION)) };
    let mut particle_system = ParticleSystem::new(1024 * 1024 * 8);
    particle_system.init_graphics_resources([128, 128, 1]);

    let mut prev_time = Instant::now();

    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let mut dx = 0;
        let mut dy = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::TextInput { text, .. } => if text == " " {
                    pause_dt = !pause_dt;
                },
                Event::MouseMotion { xrel, yrel, .. } => if mouse_state.left() {
                    dx = xrel;
                    dy = yrel;
                },
                _ => {}
            }
        }

        let time_now = Instant::now();
        let mut dt_sec = (time_now - prev_time).as_secs_f64();
        prev_time = time_now;

        let keyboard_state = event_pump.keyboard_state();
        handle_input(&mut cam, &keyboard_state, dx, dy, dt_sec as f32);

        if keyboard_state.is_scancode_pressed(Scancode::LCtrl)
            && keyboard_state.is_scancode_pressed(Scancode::R)
        {
            particle_system.load_shaders();
        }

        if pause_dt {
            dt_sec = 0.0
        }

        particle_system.update(dt_sec);

        render(&mut particle_system, &cam);
        window.gl_swap_window();
    }
}


trait Miliseconds {
    fn as_milis(&self) -> u64;
    fn as_secs_f64(&self) -> f64;
}


impl Miliseconds for std::time::Duration {
    fn as_milis(&self) -> u64 {
        (self.as_secs() as f64 * 1000.0 + self.subsec_nanos() as f64 * 1e-6) as u64
    }

    fn as_secs_f64(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 * 1e-9
    }
}
