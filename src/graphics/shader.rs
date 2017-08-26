use gl;
use std;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ops::Drop;


#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    Compute
}


#[derive(Debug, Default)]
pub struct Shader {
    source_file: String,
    gl_handle: u32,
    shader_type: ShaderType,
}


#[derive(Debug, Default)]
pub struct ShaderProgram {
    gl_handle: u32
}


impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            gl_handle: unsafe{ gl::CreateProgram() }
        }
    }

    pub fn attach_shader(&mut self, shader: &Shader) {
        unsafe {
            let t = get_gl_shader_type(&shader.shader_type);
            println!("Attaching shader of type {}, handle:{} to program {}",
                t, shader.gl_handle, self.gl_handle);
            gl::AttachShader(self.gl_handle, shader.gl_handle);
        }
    }

    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.gl_handle);
            
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(self.gl_handle, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                println!("Failed to link: {} ", self.gl_handle);
            }
            else {
                println!("Linked successfully {}", self.gl_handle);
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.gl_handle);
        }
    }
}


impl Shader {
    pub fn new (shader_type: ShaderType, source_file: &str) -> Shader {
        Shader {
            source_file: source_file.to_string(),
            gl_handle: 0,
            shader_type: shader_type,
        }
    }

    pub fn compile(&mut self) {
        unsafe {
            let shader_type = get_gl_shader_type(&self.shader_type);
            println!("Creating shader of type: {} Vtx={} Fragment={}", shader_type, gl::VERTEX_SHADER, gl::FRAGMENT_SHADER);
            self.gl_handle = gl::CreateShader(shader_type);
            
            let file_buf = self.read_shader_file();

            let shader_str = CString::new(file_buf).unwrap();

            gl::ShaderSource(self.gl_handle, 1, &shader_str.as_ptr(), std::ptr::null());
            
            gl::CompileShader(self.gl_handle);
        }
    }

    fn read_shader_file(&self) -> Vec<u8> {
        let mut file = File::open(&self.source_file)
            .expect("ERROR: Shader file not found!");
        
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        
        file_buf
    }
}


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_handle);
        }
    }
}


fn get_gl_shader_type(shader_type: &ShaderType) -> gl::types::GLenum {
    match *shader_type {
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
        ShaderType::Geometry => gl::GEOMETRY_SHADER,
        ShaderType::Compute => gl::COMPUTE_SHADER,
    }
}


impl Default for ShaderType {
    fn default() -> Self {
        ShaderType::Vertex
    }
}