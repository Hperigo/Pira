
use crate::gl;

use std::ffi::c_void;

pub struct Fbo{
    fbo_handle : gl::types::GLuint,
    texture_handle : gl::types::GLuint,
}

impl Fbo{
    pub fn new() -> Self{
        
        let mut fbo = Fbo {
            fbo_handle : 0,
            texture_handle : 0,
        };

        unsafe {
            gl::GenFramebuffers(1, &mut fbo.fbo_handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo.fbo_handle);
            
            gl::GenTextures(1, &mut fbo.texture_handle);
            gl::BindTexture(gl::TEXTURE_2D, fbo.texture_handle);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 800, 600, 0, gl::RGB, gl::UNSIGNED_BYTE, 0 as *const c_void);

            gl::TexParameteri( gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32 );
            gl::TexParameteri( gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32 );

            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, fbo.texture_handle, 0);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("Error framebuffer not complete...");
            }

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        return fbo;
    }

    pub fn bind(&self){

    }

    pub fn unbind(&self){

    }
}

impl Drop for Fbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo_handle);
        }
    }
}