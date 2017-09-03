use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader::Shader;
use shader::ShaderProgram;
use shader::ShaderType;
use super::graphics::vao::VAO;
use super::Miliseconds;
use cgmath;
use cgmath::SquareMatrix;
use cgmath::Matrix4;
use camera::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}


pub struct ParticleSystem {
    particle_pos: Vec<Position>,
    particle_vel: Vec<Velocity>,
    draw_shader_program: ShaderProgram,
    compute_shader_program: ShaderProgram,
    now: std::time::Instant,
    position_buffer_compute_handle: u32,
    velocity_buffer_compute_handle: u32,
    draw_vao_handle: u32,
    compute_shader_work_groups: [u32; 3]
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particle_pos: Vec::with_capacity(particle_count),
            particle_vel: Vec::with_capacity(particle_count),
            draw_shader_program: ShaderProgram::new(),
            compute_shader_program: ShaderProgram::new(),
            now: std::time::Instant::now(),
            position_buffer_compute_handle: 0,
            velocity_buffer_compute_handle: 0,
            draw_vao_handle: 0,
            compute_shader_work_groups: [0, 0, 0]
        };

        let mut rng = rand::thread_rng();
        let range = Range::new(-1000.0, 1000.0);
        
        system.particle_pos.reserve(particle_count);
        system.particle_vel.reserve(particle_count);

        for _ in 0..particle_count {
            let particle = Position {
                x : range.ind_sample(&mut rng),
                y : range.ind_sample(&mut rng),
                z : 0.0, //range.ind_sample(&mut rng),
                w : 0.0
            };

            system.particle_pos.push(particle);
            system.particle_vel.push(Velocity{x: 0.0, y: 0.0, z: 0.0, w:0.0});
        }

        system
    }

    pub fn init_graphics_resources(&mut self, work_groups: [u32; 3]) {     

         self.position_buffer_compute_handle = ParticleSystem::allocate_buffer(gl::SHADER_STORAGE_BUFFER, 
             self.particle_pos.as_ptr() as *const _, (self.particle_pos.len() * std::mem::size_of::<Position>()) as isize);
        self.velocity_buffer_compute_handle = ParticleSystem::allocate_buffer(gl::SHADER_STORAGE_BUFFER, 
             self.particle_vel.as_ptr() as *const _, (self.particle_vel.len() * std::mem::size_of::<Velocity>()) as isize);
        
        self.compute_shader_work_groups = work_groups;
        
        unsafe {
            gl::GenVertexArrays(1, &mut self.draw_vao_handle);
            gl::BindVertexArray(self.draw_vao_handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer_compute_handle);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        println!("Position buffer handle: {}, Velocity buffer handle: {}", self.position_buffer_compute_handle, self.velocity_buffer_compute_handle);

        let mut vertex_shader = Shader::new(ShaderType::Vertex, "shaders/vertex_shader.v.glsl");
        vertex_shader.compile();

        let mut fragment_shader = Shader::new(ShaderType::Fragment, "shaders/pixel_shader.p.glsl");
        fragment_shader.compile();

        let mut geometry_shader = Shader::new(ShaderType::Geometry, "shaders/geometry_shader.g.glsl");
        geometry_shader.compile();

        self.draw_shader_program.attach_shader(&vertex_shader);
        self.draw_shader_program.attach_shader(&fragment_shader);
        self.draw_shader_program.attach_shader(&geometry_shader);
        self.draw_shader_program.link();

        let mut compute_shader = Shader::new(ShaderType::Compute, "shaders/compute_shader.c.glsl");
        compute_shader.compile();

        self.compute_shader_program.attach_shader(&compute_shader);
        self.compute_shader_program.link();
    }
  
    pub fn update(&mut self, dt: f64) {

        self.compute_shader_program.start_use();
        {
            self.compute_shader_program.set_uniform_1f("dt", dt as f32);
            
            let count = self.particle_pos.len();
            self.compute_shader_program.set_uniform_1i("g_NumParticles", count as i32);

            let size_in_bytes = count * std::mem::size_of::<Position>();
            unsafe {
                gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER, 0, 
                    self.position_buffer_compute_handle, 0, size_in_bytes as isize);
                gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER, 1, 
                    self.velocity_buffer_compute_handle, 0, size_in_bytes as isize);

                gl::DispatchCompute(self.compute_shader_work_groups[0], self.compute_shader_work_groups[1], self.compute_shader_work_groups[2]);
                gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
            }

        }
        self.compute_shader_program.stop_use();
    }

    fn allocate_buffer(buffer_type: gl::types::GLenum, data: *const std::os::raw::c_void, size: isize) -> u32 {
        let mut buffer_object = 0u32;
        
        unsafe {
            gl::GenBuffers(1, &mut buffer_object);

            gl::BindBuffer(buffer_type, buffer_object);

            gl::BufferData(buffer_type, size, data, gl::STATIC_DRAW);

            gl::BindBuffer(buffer_type, 0);
        }

        buffer_object
    }


    pub fn render(&mut self) {

        self.draw_shader_program.start_use();

        let elapsed = self.now.elapsed().as_milis();
        let colorg = (elapsed % 1000) as f32 / 1000.0f32;
        self.draw_shader_program.set_uniform4f("vtx_color", &[0.3, colorg, 0.3, 1.0]);
        
        let identity_mtx = cgmath::Matrix4::<f32>::identity();
        let cam = Camera::new();
        self.draw_shader_program.set_uniform_matrix4("view_from_world", cam.view_from_world.as_ref());
        self.draw_shader_program.set_uniform_matrix4("proj_from_view", cam.proj_from_view.as_ref());

        unsafe {
            gl::BindVertexArray(self.draw_vao_handle);
            gl::DrawArrays(gl::POINTS, 0, self.particle_pos.len() as i32);
            gl::BindVertexArray(0);
        }    

        self.draw_shader_program.stop_use();
    }
}