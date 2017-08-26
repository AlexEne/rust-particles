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

    pub fn set_buffer(&self, data : &Vec<f32>, location: u32) {
        self.bind();
        let mut gl_handle = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut gl_handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_handle);

            gl::BufferData(gl::ARRAY_BUFFER, 
                (data.len() * 4) as isize, 
                data.as_ptr() as *const _, gl::STATIC_DRAW);

            //Describe data at that location
            gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE, 3*4, std::ptr::null());
            
            //Enable vertex attrib at location
            //This is the same as "location = bla", in the vertex shader code.
            gl::EnableVertexAttribArray(location);
        }
        self.unbind();
    }
}