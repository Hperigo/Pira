pub mod glsl_prog;
pub use self::glsl_prog::GlslProg;

pub mod stock_shader;
pub use self::stock_shader::StockShader;

pub mod vbo;
pub use self::vbo::Vbo;

pub mod vao;
pub use self::vao::Vao;
pub use self::vao::VertexAttrib;

pub mod texture;
pub use self::texture::Texture;

use crate::gl;

pub fn clear( red : f32, green : f32, blue : f32, alpha : f32 ){
    unsafe{
        //gl::Enable(gl::DEPTH_TEST); 
        gl::ClearColor(red, green, blue, alpha);
        gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
    }
}


pub fn set_window_matrices( x : i32, y : i32, width : i32, height : i32 ){
    unsafe{
        gl::Viewport( x, y, width, height );
    }
}