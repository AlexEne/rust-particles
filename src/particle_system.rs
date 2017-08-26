use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader::Shader;
use shader::ShaderProgram;
use shader::ShaderType;
use super::graphics::vao::VAO;
use super::Miliseconds;

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
    vao: VAO,
    now: std::time::Instant
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(particle_count),
            draw_shader_program: ShaderProgram::new(),
            vao: VAO::new(),
            now: std::time::Instant::now()
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
        
        self.vao.set_buffer(&vertices, 0);

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
        let elapsed = self.now.elapsed().as_milis();
        let colorg = (elapsed % 1000) as f32 / 1000.0f32;
        self.draw_shader_program.set_uniform4f("vtxColor", &[0.3, colorg, 0.3, 1.0]);
        unsafe {
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }    
    }
}

