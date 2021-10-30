pub mod vbo;
pub use self::vbo::Vbo;

pub mod vao;
pub use self::vao::Vao;
pub use self::vao::VertexAttrib;

pub mod stock_shader;
pub use self::stock_shader::StockShader;

pub mod glsl_prog;
pub use self::glsl_prog::GlslProg;


/*




pub mod texture;
pub use self::texture::Texture;

pub mod fbo;
pub use self::fbo::Fbo;
pub use self::fbo::FboSettings;
*/

use glow::*;

pub fn clear( gl : &glow::Context, red : f32, green : f32, blue : f32, alpha : f32 ){
    unsafe{
        gl.clear_color(red, green, blue, alpha);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }
}


pub fn set_viewport(gl : &glow::Context, x : i32, y : i32, width : i32, height : i32 ){
    unsafe{
        gl.viewport(x,y,width, height);
    }
}