use gl;
use std;
use std::ffi::CString;


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
            
            let shader_str = CString::new(self.source.as_bytes()).unwrap();
            gl::ShaderSource(self.object_id, 1, &shader_str.as_ptr(), std::ptr::null());
            
            gl::CompileShader(self.object_id);

            let mut status = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(self.object_id, gl::COMPILE_STATUS, &mut status);
            
            if status != (gl::TRUE as gl::types::GLint) {
                let mut len = 0;
                gl::GetShaderiv(self.object_id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(self.object_id,
                                     len,
                                     std::ptr::null_mut(),
                                     buf.as_mut_ptr() as *mut gl::types::GLchar);
                
                println!("Failed to compile :\n{}", self.source);                 
                panic!("{}", std::str::from_utf8(&buf).unwrap());
            }
            else {
                println!("Shader compiled successfully! \n {}", self.source);
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