use gl;
use image;
use std::ffi::c_void;

pub struct Texture{
    pub handle: gl::types::GLuint,
    gl_type : gl::types::GLuint,
}

impl Texture{

    //TODO: add options ( mag_filter, image type... )
    pub fn new_from_image(img : &image::RgbaImage  ) -> Texture {

        // let (width, height) = img.dimensions();
        let mut texture_handle = 0;
       unsafe {
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
       }

        Texture {
            handle : texture_handle,
            gl_type : gl::TEXTURE_2D
        }
    }



    pub fn bind(&self){
        unsafe{
            gl::BindTexture(self.gl_type, self.handle);
        }
    }

    pub fn unbind(&self){
        unsafe{
            gl::BindTexture(self.gl_type, 0);
        }
    }
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
          gl::DeleteTextures(1, &mut self.handle );
        }
    }
}
