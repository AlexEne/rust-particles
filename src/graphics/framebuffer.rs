use gl;
use graphics::texture::Texture;
use std::ops::Drop;

pub struct FrameBuffer {
    gl_handle: u32,
    color_buffer: Texture,
    highlights: Texture
}


impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        let mut frame_buffer = FrameBuffer {
            gl_handle: 0,
            color_buffer: Texture::new(width, height),
            highlights: Texture::new(width, height)
        };
        unsafe {
            gl::GenFramebuffers(1, &mut frame_buffer.gl_handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer.gl_handle);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                frame_buffer.color_buffer.gl_handle,
                0,
            );

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT1,
                gl::TEXTURE_2D,
                frame_buffer.highlights.gl_handle,
                0,
            );

            let mut render_buffer_obj = 0u32;
            gl::GenRenderbuffers(1, &mut render_buffer_obj);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer_obj);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                render_buffer_obj,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer setup failed");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        frame_buffer
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn get_color_texture(&mut self) -> &mut Texture {
        &mut self.color_buffer
    }

    pub fn get_highlights_texture(&mut self) -> &mut Texture {
        &mut self.highlights
    }
}


impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.gl_handle);
        }
    }
}
