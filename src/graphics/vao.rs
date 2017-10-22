use gl;
use std;

//TODO
//Do a derive(GLResource) that adds gl_handle to types.
//Is that even possible?
#[derive(Debug, Default)]
pub struct VertexArrayObj {
    gl_handle: u32,
}


#[derive(Debug, Default)]
pub struct VertexBufferObj {
    gl_handle: u32,
}

impl VertexBufferObj {
    pub fn new() -> VertexBufferObj {
        let mut vbo = VertexBufferObj { gl_handle: 0 };

        unsafe {
            gl::GenBuffers(1, &mut vbo.gl_handle);
        };

        vbo
    }

    pub fn set_buffer_data_from_raw_ptr(&mut self, data: *const std::os::raw::c_void, size: isize) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_handle);

            gl::BufferData(gl::ARRAY_BUFFER, size as isize, data, gl::STATIC_DRAW);
        }
    }

    pub fn set_buffer_data(&mut self, data: &[f32]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_handle);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * data.len()) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }

    //Enable vertex attrib at location and describe the data there
    //This is the same as "location = bla", in the vertex shader code.
    pub fn describe_data(&self, location: u32, component_count: i32, stride: usize, offset: usize) {
        unsafe {
            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(
                location,
                component_count,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset as *const _,
            );
        }
    }

    pub fn gl_handle(&self) -> u32 {
        self.gl_handle
    }
}

impl VertexArrayObj {
    pub fn new() -> VertexArrayObj {
        let mut vao: VertexArrayObj = VertexArrayObj::default();

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
}

impl std::ops::Drop for VertexArrayObj {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.gl_handle);
        }
    }
}

impl std::ops::Drop for VertexBufferObj {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_handle);
        }
    }
}
