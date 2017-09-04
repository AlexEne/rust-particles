use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader::Shader;
use shader::ShaderProgram;
use shader::ShaderType;
use graphics::framebuffer::FrameBuffer;
use super::Miliseconds;
use camera::Camera;
use graphics::vao::VertexBufferObj;
use graphics::vao::VertexArrayObj;

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
    possition_vbo: VertexBufferObj,
    velocity_vbo: VertexBufferObj,
    draw_vao: VertexArrayObj,
    screen_vao: VertexArrayObj,
    compute_shader_work_groups: [u32; 3],
    frame_buffer: FrameBuffer,
    screen_program: ShaderProgram,
    fullscreen_quad_vbo: VertexBufferObj
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particle_pos: Vec::with_capacity(particle_count),
            particle_vel: Vec::with_capacity(particle_count),
            draw_shader_program: ShaderProgram::new(),
            compute_shader_program: ShaderProgram::new(),
            now: std::time::Instant::now(),
            possition_vbo: VertexBufferObj::new(),
            velocity_vbo: VertexBufferObj::new(),
            draw_vao: VertexArrayObj::new(),
            screen_vao: VertexArrayObj::new(),
            compute_shader_work_groups: [0, 0, 0],
            frame_buffer: FrameBuffer::new(1600, 900),
            screen_program: ShaderProgram::new(),
            fullscreen_quad_vbo: VertexBufferObj::new()
        };

        let mut rng = rand::thread_rng();
        let range = Range::new(-1000.0, 1000.0);
        
        system.particle_pos.reserve(particle_count);
        system.particle_vel.reserve(particle_count);

        for _ in 0..particle_count {
            let particle = Position {
                x : range.ind_sample(&mut rng),
                y : range.ind_sample(&mut rng),
                z : range.ind_sample(&mut rng),
                w : 0.0
            };

            system.particle_pos.push(particle);
            system.particle_vel.push(Velocity{x: 0.0, y: 0.0, z: 0.0, w:0.0});
        }

        system
    }

    pub fn init_graphics_resources(&mut self, work_groups: [u32; 3]) {     
        self.compute_shader_work_groups = work_groups;
        
        self.draw_vao.bind();
        let count = self.particle_pos.len();
        let size = count * std::mem::size_of::<f32>();
        self.possition_vbo.set_buffer_data_from_raw_ptr(self.particle_pos.as_ptr() as *const _, size as isize);
        self.possition_vbo.describe_data(0, 4, 4*std::mem::size_of::<f32>(), 0);
        self.draw_vao.unbind();


        let quad_vertices: [f32; 24] = [ // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
            // positions   // texCoords
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
             1.0, -1.0,  1.0, 0.0,

            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0
        ];
        self.screen_vao.bind();
        self.fullscreen_quad_vbo.set_buffer_data(&quad_vertices);
        self.fullscreen_quad_vbo.describe_data(0, 2, 4 * std::mem::size_of::<f32>(), 0);
        self.fullscreen_quad_vbo.describe_data(1, 2, 4 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>());
        self.screen_vao.unbind();

        self.velocity_vbo.set_buffer_data_from_raw_ptr(self.particle_vel.as_ptr() as *const _, size as isize);

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

        let mut vertex_shader = Shader::new(ShaderType::Vertex, "shaders/fullscreen_quad.v.glsl");
        vertex_shader.compile();

        let mut fragment_shader = Shader::new(ShaderType::Fragment, "shaders/copy_texture_to_quad.p.glsl");
        fragment_shader.compile();

        self.screen_program.attach_shader(&vertex_shader);
        self.screen_program.attach_shader(&fragment_shader);
        self.screen_program.link();
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
                    self.possition_vbo.gl_handle(), 0, size_in_bytes as isize);
                gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER, 1, 
                    self.velocity_vbo.gl_handle(), 0, size_in_bytes as isize);

                gl::DispatchCompute(self.compute_shader_work_groups[0], self.compute_shader_work_groups[1], self.compute_shader_work_groups[2]);
                gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
            }

        }
        self.compute_shader_program.stop_use();
    }



    pub fn render_particles(&mut self, cam: &Camera) {

        self.draw_shader_program.start_use();

        let elapsed = self.now.elapsed().as_milis();
        let colorg = (elapsed % 1000) as f32 / 1000.0f32;
        self.draw_shader_program.set_uniform4f("vtx_color", &[0.3, colorg, 0.3, 1.0]);
        
        self.draw_shader_program.set_uniform_matrix4("view_from_world", cam.view_from_world.as_ref());
        self.draw_shader_program.set_uniform_matrix4("proj_from_view", cam.proj_from_view.as_ref());

        unsafe {
           self.draw_vao.bind();
            gl::DrawArrays(gl::POINTS, 0, self.particle_pos.len() as i32);
            self.draw_vao.unbind();
        }    
        self.draw_shader_program.stop_use();
    }

    pub fn render(&mut self, cam: &Camera) {
        
        //First pass
        self.frame_buffer.bind();
        unsafe { 
            gl::Viewport(0, 0, 1600, 900);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
        }
        self.render_particles(cam);    
        self.frame_buffer.unbind();

        //Second pass
        unsafe{ 
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.screen_program.start_use();
        unsafe {
            self.screen_vao.bind();
            gl::Disable(gl::DEPTH_TEST);
            gl::BindTexture(gl::TEXTURE_2D, self.frame_buffer.get_texture_color_buffer());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            self.screen_vao.unbind();
        }
        self.screen_program.stop_use();
    }
}