extern crate nalgebra_glm as glm;
use crate::gl_helper::Bindable;
use glow::{self, HasContext};

pub struct FboSettings {
    pub width: i32,
    pub height: i32,
    pub depth: bool,
}

pub struct Fbo {
    pub fbo_handle: Option<glow::Framebuffer>,
    pub texture_handle: Option<glow::Texture>,

    settings: FboSettings,
}

impl Fbo {
    pub fn new(gl: &glow::Context, settings: FboSettings) -> Self {
        let (fbo, texture) = unsafe {
            let fbo = gl
                .create_framebuffer()
                .expect("could not create frame buffer");

            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));

            let texture = gl
                .create_texture()
                .expect("could not create frame buffer texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as i32,
                settings.width,
                settings.height,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                None,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );

            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                println!("Error creating framebuffer");
            }

            gl.bind_texture(glow::TEXTURE_2D, None);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            (fbo, texture)
        };

        Self {
            fbo_handle: Some(fbo),
            texture_handle: Some(texture),
            settings,
        }
    }

    pub fn bind_texture(&self, gl: &glow::Context) {
        unsafe {
            assert_eq!(
                self.texture_handle.is_some(),
                true,
                "You are trying to bind a NONE texture"
            );
            gl.bind_texture(glow::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn unbind_texture(&self, gl: &glow::Context) {
        unsafe {
            // gl::BindTexture(gl::TEXTURE_2D, 0);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn get_width(&self) -> i32 {
        self.settings.width
    }

    pub fn get_height(&self) -> i32 {
        self.settings.height
    }

    pub fn get_texture_handle(&self) -> Option<glow::Texture> {
        return self.texture_handle;
    }
}

impl Bindable for Fbo {
    fn bind(&self, gl: &glow::Context) {
        unsafe {
            assert_eq!(
                self.fbo_handle.is_some(),
                true,
                "You are trying to bind a NONE texture"
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, self.fbo_handle);
        }
    }

    fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
}
