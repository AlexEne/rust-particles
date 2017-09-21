use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader;
use shader::Shader;
use shader::ShaderInputData;
use shader::ShaderProgram;
use shader::ShaderType;
use graphics::framebuffer::FrameBuffer;
use super::Miliseconds;
use camera::Camera;
use graphics::vao::VertexBufferObj;
use graphics::vao::VertexArrayObj;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

struct ColliderData {
    sphere_radius: [f32; 20],
    sphere_positions: [Vec3; 20]
}

pub struct ParticleSystem {
    particle_pos: Vec<Vec4>,
    particle_vel: Vec<Vec4>,
    draw_shader_program: ShaderProgram,
    compute_shader_program: ShaderProgram,
    now: std::time::Instant,
    possition_vbo: VertexBufferObj,
    velocity_vbo: VertexBufferObj,
    draw_vao: VertexArrayObj,
    screen_vao: VertexArrayObj,
    compute_shader_work_groups: [u32; 3],
    frame_buffer: FrameBuffer,
    blur_frame_buffers: [FrameBuffer; 2],
    blur_shader: ShaderProgram,
    screen_program: ShaderProgram,
    fullscreen_quad_vbo: VertexBufferObj,
    collider_data: ColliderData
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
            blur_frame_buffers: [FrameBuffer::new(1600, 900), FrameBuffer::new(1600, 900)],
            screen_program: ShaderProgram::new(),
            blur_shader: ShaderProgram::new(),
            fullscreen_quad_vbo: VertexBufferObj::new(),
            collider_data: ColliderData::new()
        };

        let mut rng = rand::thread_rng();
        let range = Range::new(-1000.0, 1000.0);
        
        system.particle_pos.reserve(particle_count);
        system.particle_vel.reserve(particle_count);

        for _ in 0..particle_count {
            let particle = Vec4 {
                x : range.ind_sample(&mut rng),
                y : range.ind_sample(&mut rng),
                z : range.ind_sample(&mut rng),
                w : 0.0
            };

            system.particle_pos.push(particle);
            system.particle_vel.push(Vec4{x: 0.0, y: 0.0, z: 0.0, w:0.0});
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

        self.load_shaders();
    }

    pub fn load_shaders(&mut self) {
        println!("Loading shaders!");

        let input = [ShaderInputData::new(ShaderType::Vertex, "shaders/vertex_shader.v.glsl"),
            ShaderInputData::new(ShaderType::Fragment, "shaders/pixel_shader.p.glsl"),
            ShaderInputData::new(ShaderType::Geometry, "shaders/geometry_shader.g.glsl")];
        self.draw_shader_program = shader::create_shader_from(&input);

        let input = [ShaderInputData::new(ShaderType::Compute, "shaders/compute_shader.c.glsl")];
        self.compute_shader_program = shader::create_shader_from(&input);

        let input = [ShaderInputData::new(ShaderType::Vertex, "shaders/fullscreen_quad.v.glsl"),
            ShaderInputData::new(ShaderType::Fragment, "shaders/copy_texture_to_quad.p.glsl")];
        self.screen_program = shader::create_shader_from(&input);

        let input = [ShaderInputData::new(ShaderType::Vertex, "shaders/fullscreen_quad.v.glsl"),
            ShaderInputData::new(ShaderType::Fragment, "shaders/blur_shader.p.glsl")];

        self.blur_shader = shader::create_shader_from(&input);
    }
  
    pub fn update(&mut self, dt: f64) {

        self.compute_shader_program.bind();
        {
            self.compute_shader_program.set_uniform_1f("dt", dt as f32);
            
            let count = self.particle_pos.len();
            self.compute_shader_program.set_uniform_1i("g_NumParticles", count as i32);

            self.compute_shader_program.set_uniform_1fv("sphereRadius", 20, &self.collider_data.sphere_radius);
            unsafe {
                let sphere_positions_buffer = std::slice::from_raw_parts(self.collider_data.sphere_positions.as_ptr() as *const f32, 60);
                self.compute_shader_program.set_uniform_3fv("sphereOffsets", 20, sphere_positions_buffer);
            }
            
            let size_in_bytes = count * std::mem::size_of::<Vec4>();
            unsafe {
                gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER, 0, 
                    self.possition_vbo.gl_handle(), 0, size_in_bytes as isize);
                gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER, 1, 
                    self.velocity_vbo.gl_handle(), 0, size_in_bytes as isize);

                gl::DispatchCompute(self.compute_shader_work_groups[0], 
                    self.compute_shader_work_groups[1], self.compute_shader_work_groups[2]);
                gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
                gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
            }

        }
        self.compute_shader_program.unbind();
    }

    pub fn render_particles(&mut self, cam: &Camera) {

        self.draw_shader_program.bind();

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
        self.draw_shader_program.unbind();
    }

    pub fn render(&mut self, cam: &Camera) {
        
        //First pass
        self.frame_buffer.bind();
        unsafe {
            gl::Viewport(0, 0, 1600, 900);
            gl::Enable(gl::DEPTH_TEST);
            
            let attachments: [u32; 2] = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1];
            gl::DrawBuffers(2, attachments.as_ptr() as * const _);

            let color: [f32; 4] = [0.003, 0.003, 0.003, 1.0];
            let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            gl::ClearBufferfv(gl::COLOR, 0, color.as_ptr() as *const _);
            gl::ClearBufferfv(gl::COLOR, 1, black.as_ptr() as *const _);
            //gl::ClearColor(0.003, 0.003, 0.003, 1.0); 
            gl::Clear(gl::DEPTH_BUFFER_BIT); 
        }

        self.render_particles(cam);    
        self.frame_buffer.unbind();

        {
            self.blur_shader.bind();
            self.screen_vao.bind();
            unsafe { 
                gl::Disable(gl::DEPTH_TEST);
            }

            //Do the blur
            let blur_passes = 5;
            let mut vertical = false;
            let mut first_pass = true;
            for _ in 0..blur_passes {
                let (source, dest) = match vertical {
                    false => (0, 1),
                    true => (1, 0)
                };
                
                self.blur_frame_buffers[dest].bind();
                let mut color_buffer = self.blur_frame_buffers[source].get_color_texture();
                if first_pass {
                    color_buffer = self.frame_buffer.get_highlights_texture();
                    first_pass = false;
                }
                color_buffer.bind();
                self.blur_shader.set_uniform_1i("vertical", vertical as i32);
                unsafe {
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }

                color_buffer.unbind();
                
                vertical = !vertical;
            }

            self.screen_vao.unbind();
            self.blur_shader.unbind();
        }

        //Final pass
        unsafe{ 
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.screen_program.bind();
        unsafe {
            self.screen_vao.bind();
            gl::Disable(gl::DEPTH_TEST);
            self.screen_program.set_uniform_1i("screenTexture", 0);
            self.screen_program.set_uniform_1i("bloom", 1);

            let color_buffer = self.frame_buffer.get_color_texture();
            gl::ActiveTexture(gl::TEXTURE0);
            color_buffer.bind();

            let blured_texture = self.blur_frame_buffers[1].get_color_texture();
            gl::ActiveTexture(gl::TEXTURE1);
            blured_texture.bind();

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            
            color_buffer.unbind();
            self.screen_vao.unbind();
        }
        self.screen_program.unbind();
    }
}


impl ColliderData {
    fn new() -> ColliderData {
        let mut colider_data = ColliderData {
            sphere_positions: [Vec3{x: 0.0, y: 0.0, z: 0.0}; 20],
            sphere_radius: [0.0; 20]
        };

        let mut rng = rand::thread_rng();
        let position_range = Range::new(-1000.0, 1000.0);
        let radius_range = Range::new(100, 400);

        for i in 0 .. 20 {
            let pos = &mut colider_data.sphere_positions[i];
            pos.x = position_range.ind_sample(&mut rng) as f32;
            pos.y = 0.0;
            pos.z = position_range.ind_sample(&mut rng) as f32;
            colider_data.sphere_radius[i] = radius_range.ind_sample(&mut rng) as f32;
        }

        colider_data
    }
}