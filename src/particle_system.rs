use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use rand;

#[derive(Debug)]
struct Particle {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

pub struct ParticleSystem {
    particles: Vec<Particle>
}

impl ParticleSystem {
    pub fn new(particle_count: usize) -> Self {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(particle_count)
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
    
    }
}
