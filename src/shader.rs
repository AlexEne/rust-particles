use gl;

#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute
}

#[derive(Debug, Default)]
pub struct Shader {
    source: String,
    object_id: u32,
    shader_type: ShaderType,
}


impl Shader {
    pub fn new (shader_type: ShaderType, source: &str) -> Shader {
        Shader {
            source: source.to_string(),
            object_id: 0,
            shader_type: shader_type,
        }
    }

    pub fn compile(&mut self) {
        unsafe {
            self.object_id = gl::CreateShader(self.get_gl_shader_type());
            
            let mut success = 0i32;
            gl::GetShaderiv(self.object_id, gl::COMPILE_STATUS, &mut success);
            
            if success != 1 {
                let mut info_log = String::with_capacity(256);
                let mut error_size = 0i32;
                gl::GetShaderInfoLog(self.object_id, 512, &mut error_size, info_log.as_ptr() as _);
                println!("Error shader compilation failed with error: {:?} for:\n {:?}", 
                    info_log, self.source);
            }
            else {
                println!("Shader compiled successfully!");
            }
        }
    }

    fn get_gl_shader_type(&self) -> gl::types::GLenum {
        match &self.shader_type {
            Vertex => gl::VERTEX_SHADER,
            Fragment => gl::FRAGMENT_SHADER,
            Compute => gl::COMPUTE_SHADER
        }
    }
}


impl Default for ShaderType {
    fn default() -> Self {
        ShaderType::Vertex
    }
}