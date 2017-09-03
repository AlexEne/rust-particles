use gl;
use std;

//TODO
//Do a derive(GLResource) that adds gl_handle to types.
//Is that even possible?
#[derive(Debug, Default)]
pub struct VAO {
    gl_handle: u32
}


#[derive(Debug, Default)]
pub struct VBO {
    gl_handle: u32,
    location: u32
}

impl VAO {
    pub fn new() -> VAO {
        let mut vao : VAO = VAO::default();
        
        unsafe {
            gl::GenVertexArrays(1, &mut vao.gl_handle);
        }

        vao
    }

    pub fn bind(&self) {
        unsafe { 
            gl::BindVertexArray(self.gl_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe { 
            gl::BindVertexArray(0);
        }
    }

    pub fn create_buffer() -> u32 {
        let mut gl_handle = 0u32;
        unsafe { gl::GenBuffers(1, &mut gl_handle); }
        gl_handle
    }

    pub fn set_buffer(&self, buffer_handle: u32, data : &[f32], location: u32, stride: u32) {
        self.bind();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_handle);

            gl::BufferData(gl::ARRAY_BUFFER, 
                (data.len() * 4) as isize, 
                data.as_ptr() as *const _, gl::STREAM_DRAW);

            //Describe data at that location
            gl::VertexAttribPointer(location, 4, 
                gl::FLOAT, gl::FALSE, stride as i32, std::ptr::null());
            
            //Enable vertex attrib at location
            //This is the same as "location = bla", in the vertex shader code.
            gl::EnableVertexAttribArray(location);
        }
        self.unbind();
    }
}