extern crate image;
use glow::{self, HasContext};
use image::EncodableLayout;


pub struct TextureSettings {
    pub mag_filter : u32,
    pub min_filter : u32,

    pub wrap_s : u32,
    pub wrap_r : u32,
}

impl TextureSettings {

    pub fn default() -> Self {
        Self{ 
            mag_filter : glow::LINEAR,
            min_filter : glow::LINEAR,

            wrap_r : glow::REPEAT,
            wrap_s : glow::REPEAT,
        }
    }
}

pub struct Texture{
    pub handle: Option<glow::Texture>,
}

impl Texture{

    //TODO: add options ( mag_filter, image type... )
    pub fn new_from_image_rgbau8(gl : &glow::Context, img : &image::RgbaImage  ) -> Self {
       Self::new_from_data(gl, img.as_bytes(), img.width() as i32, img.height() as i32, glow::RGBA)
    }

    pub fn new_from_data( gl : &glow::Context, data : &[u8], width : i32, height : i32, format : u32 ) -> Self {
        let texture_handle;
        unsafe {
            texture_handle = gl.create_texture().expect("Could not create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_handle));
 
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_R, glow::REPEAT as i32);
 
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
 
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                 0,
                 glow::RGBA as i32,
                 width,
                 height,
                 0,
                 format,
                 glow::UNSIGNED_BYTE,
                 Some( data ));
 
            gl.bind_texture(glow::TEXTURE_2D, None);
       }

        Texture {
            handle : Some(texture_handle),
        }
    }
 
    pub fn bind(&self, gl : &glow::Context){
        unsafe{
            assert_eq!(self.handle.is_some(), true, "You are trying to bind a NONE texture");
            gl.bind_texture(glow::TEXTURE_2D, self.handle);
        }
    }

    pub fn unbind(&self, gl : &glow::Context){
        unsafe{
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

}