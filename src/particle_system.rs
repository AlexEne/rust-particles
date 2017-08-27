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
    particle_pos: [Vec<Position>; 2],
    particle_vel: [Vec<Velocity>; 2],
    draw_shader_program: ShaderProgram,
    vao: VAO,
    now: std::time::Instant,
    gl_handle_pos_buffer: u32,
    active_buffer: usize
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particle_pos: [Vec::with_capacity(particle_count), Vec::with_capacity(particle_count)],
            particle_vel: [Vec::with_capacity(particle_count), Vec::with_capacity(particle_count)],
            draw_shader_program: ShaderProgram::new(),
            vao: VAO::new(),
            now: std::time::Instant::now(),
            gl_handle_pos_buffer: 0,
            active_buffer: 0
        };

        let mut rng = rand::thread_rng();
        let range = Range::new(-1.0, 1.0);

        for i in 0..particle_count {
            let particle = Position {
                x : range.ind_sample(&mut rng),
                y : range.ind_sample(&mut rng),
                z : 0.0, //range.ind_sample(&mut rng),
                w : 0.0
            };

            for i in 0..2 {
                system.particle_pos[i].push(particle);
            }
        }
        
        system
    }

    pub fn update(&mut self, dt: f64) {
        
        let active_buffer = self.active_buffer;
        let output_buffer = self.get_output_buffer_index();
        
        for i in 0..self.particle_pos[0].len() {
            let mut input_y = 0.0;
            {
                let ref input_buffer = &(self.particle_pos[active_buffer]);
                input_y = input_buffer[i].y;
            }
            
            let ref mut output_buffer = &mut (self.particle_pos[output_buffer]);
        
            output_buffer[i].y = input_y + (-0.100 * dt) as f32;
        }
        
    }

    fn get_output_buffer_index(&self) -> usize {
        ((self.active_buffer + 1) % 2) as usize
    }

    fn get_input_buffer_index(&self) -> usize {
        self.active_buffer
    }

    pub fn init_graphics_resources(&mut self) {
        let vertices: Vec<f32> = vec!(
            -0.5, -0.5, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0,
             0.0,  0.5, 0.0, 0.0
        );
        
        unsafe {
            //TODO is there another way to do this?
            let memory = std::slice::from_raw_parts(self.particle_pos.as_ptr() as *const f32, 
                self.particle_pos.len()*4);
            self.gl_handle_pos_buffer = VAO::create_buffer();    
            self.vao.set_buffer(self.gl_handle_pos_buffer, memory, 0, 4*4);
        }
        
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

    }
  
    pub fn render(&mut self) {

        self.draw_shader_program.start_use();

        let elapsed = self.now.elapsed().as_milis();
        let colorg = (elapsed % 1000) as f32 / 1000.0f32;
        self.draw_shader_program.set_uniform4f("vtx_color", &[0.3, colorg, 0.3, 1.0]);
        
        let identity_mtx = cgmath::Matrix4::<f32>::identity();
        self.draw_shader_program.set_uniform_matrix4("view_from_world", identity_mtx.as_ref());
        self.draw_shader_program.set_uniform_matrix4("proj_from_view", identity_mtx.as_ref());

        let buffer_to_draw_index = self.get_output_buffer_index();
        let ref pos_buffer_to_draw = &self.particle_pos[buffer_to_draw_index];
        unsafe {
            //TODO is there another way to do this?
            let memory = std::slice::from_raw_parts(pos_buffer_to_draw.as_ptr() as *const f32, 
                pos_buffer_to_draw.len()*4);
            self.vao.set_buffer(self.gl_handle_pos_buffer, memory, 0, 4*4);
        }
        self.vao.bind();


        unsafe {
            gl::DrawArrays(gl::POINTS, 0, pos_buffer_to_draw.len() as i32);
        }    

        self.draw_shader_program.stop_use();
        
        self.active_buffer = (self.active_buffer + 1) % 2;
    }
}