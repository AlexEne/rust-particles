use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader::Shader;
use shader::ShaderProgram;
use shader::ShaderType;

#[derive(Debug)]
struct Particle {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    draw_shader_program: ShaderProgram,
    vao_handle: u32
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(particle_count),
            draw_shader_program: ShaderProgram::new(),
            vao_handle: 0
        };

        let mut rng = rand::thread_rng();
        let range = Range::new(-1000.0, 1000.0);

        for i in 0..particle_count {
            let particle = Particle {
                x : range.ind_sample(&mut rng),
                y : range.ind_sample(&mut rng),
                z : range.ind_sample(&mut rng),
                w : 0.0
            };

            system.particles.push(particle);
        }
        
        system
    }

    pub fn update(& self, dt: f64) {
        
    }

    pub fn init_graphics_resources(&mut self) {
        let vertices: Vec<f32> = vec!(
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0
        );

        //Vertex Array Object
        // Init code
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao_handle); 
            gl::BindVertexArray(self.vao_handle);
            
            let mut vbo = 0u32;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * 4) as isize, 
                vertices.as_ptr() as *const _, gl::STATIC_DRAW);
            
            //Describe data at location 0
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3*4, std::ptr::null());
            
            //Enable vertex attrib at location 0
            //0 comes from location = 0, in the vertex shader code.
            gl::EnableVertexAttribArray(0);
        }


        let mut vertex_shader = Shader::new(ShaderType::Vertex, "shaders/vertex_shader.v.glsl");
        vertex_shader.compile();

        let mut fragment_shader = Shader::new(ShaderType::Fragment, "shaders/pixel_shader.p.glsl");
        fragment_shader.compile();

        self.draw_shader_program.attach_shader(&vertex_shader);
        self.draw_shader_program.attach_shader(&fragment_shader);
        self.draw_shader_program.link();

    }
  
    pub fn render(&self) {

        self.draw_shader_program.use_program();

        unsafe {
            gl::BindVertexArray(self.vao_handle);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }    
    }
}
