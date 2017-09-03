extern crate sdl2;
extern crate gl;
extern crate rand;
extern crate cgmath;

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


fn render(particle_system: &mut ParticleSystem) {
    unsafe { 
        gl::Viewport(0, 0, 1600, 900);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
    }

    particle_system.render();

    unsafe {
        gl::Flush();
    }
}


#[no_mangle]
pub extern "system" fn debug_callback(source: gl::types::GLenum,
        err_type: gl::types::GLenum,
        id: gl::types::GLuint,
        severity: gl::types::GLenum,
        _: gl::types::GLsizei,
        message: *const c_char,
        _: *mut c_void) {

    unsafe{
        let err_text = std::ffi::CStr::from_ptr(message);
        println!("Type: {:#x} ID: {:#x} Severity: {:#x}:\n  {:#?}", err_type, id, severity, err_text.to_str().unwrap())
    }
}


fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();

    gl_attr.set_context_version(4, 3);

    let window = video_subsystem.window("Rust SDL window", 1600, 900)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context();
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
    let mut particle_system = ParticleSystem::new(1024*1024*16);
    particle_system.init_graphics_resources([128, 128, 1]);
    
    let mut prev_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                _ => {}
            }
        }

        let time_now = Instant::now();
        let dt_sec = (time_now - prev_time).as_secs_f64();
        prev_time = time_now;
        
        particle_system.update(dt_sec);
        
        render(&mut particle_system);
        window.gl_swap_window();

        //std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
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