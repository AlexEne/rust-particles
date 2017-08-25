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
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> ParticleSystem {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(particle_count),
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

    pub fn render(&self) {
        let vertices = vec![
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0
        ];

        let mut VBO = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, 
                vertices.len() as isize,
                vertices.as_ptr() as *const _, gl::STATIC_DRAW);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        let mut vertex_shader = Shader::new(ShaderType::Vertex, "shaders/vertex_shader.v.glsl");
        vertex_shader.compile();

        let mut fragment_shader = Shader::new(ShaderType::Fragment, "shaders/pixel_shader.p.glsl");
        fragment_shader.compile();

        let mut shader_program = ShaderProgram::new();
        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);
        shader_program.link();
    }
}
