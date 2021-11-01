extern crate image;
use glow::{self, HasContext};
use image::EncodableLayout;
use std::ffi::c_void;

pub struct Texture{
    pub handle: Option<glow::Texture>,
    gl_type : u32,
}

impl Texture{

    //TODO: add options ( mag_filter, image type... )
    pub fn new_from_image(gl : &glow::Context, img : &image::RgbaImage  ) -> Texture {

        let (width, height) = img.dimensions();
        let texture_handle;
       unsafe {
           
           texture_handle = gl.create_texture().expect("Could not create texture");
           gl.bind_texture(glow::TEXTURE_2D, Some(texture_handle));

           gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
           gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_R, glow::REPEAT as i32);

           gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
           gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

           let width = img.width() as i32;
           let height = img.height() as i32;

           gl.tex_image_2d(
               glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(img.as_bytes()));

           gl.bind_texture(glow::TEXTURE_2D, None);
           /*
            gl::GenTextures(1, &mut texture_handle);
            gl::BindTexture( gl::TEXTURE_2D, texture_handle);

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let width = img.width() as i32;
            let height = img.height() as i32;
            
            gl::TexImage2D( gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
             */
       }

        Texture {
            handle : Some(texture_handle),
            gl_type : glow::TEXTURE_2D,
        }
    }
    /*
    pub fn new_from_data( data : &Vec<u8>, width : i32, height : i32, format : gl::types::GLenum ) -> Texture {
        let mut texture_handle = 0;
        println!("creating texture from data, width: {}, height {}, len {}", width, height, data.len());
      unsafe {
            gl::GenTextures(1, &mut texture_handle);
            gl::BindTexture( gl::TEXTURE_2D, texture_handle);

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let width = width as i32;
            let height = height as i32;
            
            gl::TexImage2D( gl::TEXTURE_2D, 0, format as i32, width, height, 0, format as u32, gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
       }

        Texture {
            handle : texture_handle,
            gl_type : gl::TEXTURE_2D
        }
    }
 */


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

// impl Drop for Texture{
//     fn drop(&mut self){
//         println!("Texture {} dropped", self.handle);
//         unsafe{
//           gl::DeleteTextures(1, &mut self.handle );
//         }
//     }
// }
