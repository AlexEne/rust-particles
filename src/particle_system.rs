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

    pub fn update(&mut self, dt: f64) {
        for particle in &mut self.particles {
            particle.z += (-100.0 * dt) as f32;
        }
    }

    pub fn init_graphics_resources(&mut self) {
        let vertices: Vec<f32> = vec!(
            -0.5, -0.5, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0,
             0.0,  0.5, 0.0, 0.0
        );
        
        self.vao.set_buffer(&vertices, 0, 4*4);

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
  
    pub fn render(&self) {

        self.draw_shader_program.start_use();

        let elapsed = self.now.elapsed().as_milis();
        let colorg = (elapsed % 1000) as f32 / 1000.0f32;
        self.draw_shader_program.set_uniform4f("vtx_color", &[0.3, colorg, 0.3, 1.0]);
        
        let identity_mtx = cgmath::Matrix4::<f32>::identity();
        self.draw_shader_program.set_uniform_matrix4("view_from_world", identity_mtx.as_ref());
        self.draw_shader_program.set_uniform_matrix4("proj_from_view", identity_mtx.as_ref());

        self.vao.bind();

        unsafe {
            gl::DrawArrays(gl::POINTS, 0, 3);
        }    

        self.draw_shader_program.stop_use();
    }
}

