use gl;
use std;

pub struct Texture {
    pub gl_handle: u32,
    width: u32,
    height: u32,
}


impl Texture {
    pub fn new(width: u32, height: u32) -> Texture {
        let mut texture = Texture {
            gl_handle: 0,
            width: width,
            height: height,
        };

        unsafe {
            gl::GenTextures(1, &mut texture.gl_handle);
            gl::BindTexture(gl::TEXTURE_2D, texture.gl_handle);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB16F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::FLOAT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        texture
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_handle);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
