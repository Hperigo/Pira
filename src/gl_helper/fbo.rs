use crate::gl_helper::{Bindable, Texture, texture::TextureSettings};
use glow::{self, HasContext};

#[derive(Clone, Copy)]
pub struct FboSettings {
    pub width: i32,
    pub height: i32,
    pub depth: bool,

    pub initialize_default_texture : bool,
}

pub struct Fbo {
    pub fbo_handle: Option<glow::Framebuffer>,
    
    pub texture : Option<Texture>,
    pub render_buffer : Option< glow::Renderbuffer >,

    settings: FboSettings,
    texture_settings : TextureSettings,
}

impl Fbo {
    pub fn new(gl: &glow::Context, settings: FboSettings, texture_settings : TextureSettings) -> Self {
        let fbo = unsafe {
            let fbo = gl
                .create_framebuffer()
                .expect("could not create frame buffer");

            fbo
        };

        let mut fbo = Self {
            fbo_handle: Some(fbo),
            render_buffer : None,
            texture : None,
            settings,
            texture_settings,
        };

        if settings.initialize_default_texture{
            fbo.initialize_default_texture(gl);
        }

        fbo
    }

    pub fn bind_texture(&self, gl: &glow::Context) {
        self.texture.as_ref().unwrap().bind(gl);
    }

    pub fn unbind_texture(&self, gl: &glow::Context) {
        self.texture.as_ref().unwrap().unbind(gl);
    }

    pub fn get_width(&self) -> i32 {
        self.settings.width
    }

    pub fn get_height(&self) -> i32 {
        self.settings.height
    }

    pub fn get_texture_handle(&self) -> Option<glow::Texture> {
        if self.texture.is_some() {
            self.texture.as_ref().unwrap().handle
        }else{
            None
        }
    }

    pub fn attach_texture(&self, gl : &glow::Context, texture : &Texture, attachment : u32){
        unsafe{ 
            gl.bind_framebuffer(glow::FRAMEBUFFER, self.fbo_handle);
            
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                attachment,
                glow::TEXTURE_2D,
                texture.handle,
                0,
            );
            
            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                println!("Error creating framebuffer");
            }
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn create_render_buffer(&mut self, gl : &glow::Context){
        unsafe {

            self.bind(gl);
            self.render_buffer = Some(gl.create_renderbuffer().expect("cound not create render buffer (depth buffer)"));
            
            gl.bind_renderbuffer(glow::RENDERBUFFER, self.render_buffer);
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT, self.get_width(), self.get_height());

            gl.framebuffer_renderbuffer(glow::FRAMEBUFFER, glow::DEPTH_ATTACHMENT, glow::RENDERBUFFER, self.render_buffer);

            self.unbind(gl);

            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                println!("Error creating framebuffer");
            }
        }  
    }

    fn initialize_default_texture(&mut self, gl : &glow::Context) {
        let texture = Texture::new_from_data(gl, None, self.settings.width, self.settings.height, self.texture_settings);
        self.attach_texture(gl, &texture, glow::COLOR_ATTACHMENT0);
        self.texture = Some(texture);
    }
}

impl Bindable for Fbo {
    fn bind(&self, gl: &glow::Context) {
        unsafe {
            assert!(
                self.fbo_handle.is_some(),
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
