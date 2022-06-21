extern crate image;
use glow::{self, HasContext};
use image::EncodableLayout;

use super::Bindable;

#[derive(Clone, Copy)]
pub struct TextureSettings {
    pub data_type: u32,
    pub internal_format: i32,
    pub format: u32,

    pub mag_filter: u32,
    pub min_filter: u32,

    pub wrap_s: u32,
    pub wrap_r: u32,
}

impl TextureSettings {
    pub fn default() -> Self {
        Self {
            data_type: glow::UNSIGNED_BYTE,
            internal_format: glow::RGBA8 as i32,
            format: glow::RGBA,
            mag_filter: glow::LINEAR,
            min_filter: glow::LINEAR,

            wrap_r: glow::REPEAT,
            wrap_s: glow::REPEAT,
        }
    }
}

pub struct Texture {
    pub handle: Option<glow::Texture>,
    pub width: i32,
    pub height: i32,
    pub settings: TextureSettings,
}

impl Texture {
    pub fn new_from_image_rgbau8(
        gl: &glow::Context,
        img: &image::RgbaImage,
        settings: TextureSettings,
    ) -> Self {
        Self::new_from_data(
            gl,
            Some(img.as_bytes()),
            img.width() as i32,
            img.height() as i32,
            settings,
        )
    }

    pub fn new_from_data(
        gl: &glow::Context,
        data: Option<&[u8]>,
        width: i32,
        height: i32,
        settings: TextureSettings,
    ) -> Self {
        let texture_handle;
        unsafe {
            texture_handle = gl.create_texture().expect("Could not create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_handle));

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_R, glow::REPEAT as i32);

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                settings.internal_format,
                width,
                height,
                0,
                settings.format,
                settings.data_type,
                data,
            );

            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        Texture {
            handle: Some(texture_handle),
            width,
            height,
            settings,
        }
    }

    pub fn update(&self, gl: &glow::Context, data: &[u8]) {
        self.bind(gl);
        unsafe {
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                self.width,
                self.height,
                self.settings.format,
                self.settings.data_type,
                glow::PixelUnpackData::Slice(data),
            )
        }
        self.unbind(gl);
    }
}

impl Bindable for Texture {
    fn bind(&self, gl: &glow::Context) {
        unsafe {
            assert!(
                self.handle.is_some(),
                "You are trying to bind a NONE texture"
            );
            gl.bind_texture(glow::TEXTURE_2D, self.handle);
        }
    }

    fn unbind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}
