use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use rand;
use gl;
use std;
use shader;

#[derive(Debug)]
struct Particle {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    shader_program: shader::Shader
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> Self {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(particle_count),
            shader_program: shader::Shader::default()
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
            println!("{:?}", particle);
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

        let mut VBO: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, 
                vertices.len() as isize,
                vertices.as_ptr() as *const _, gl::STATIC_DRAW);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
