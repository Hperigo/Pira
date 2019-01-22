extern crate gl;

#[derive(Clone)]
pub struct Vbo{
    handle: gl::types::GLuint,
    gl_type :  gl::types::GLuint
}

impl Vbo{

    pub fn new( data : &Vec<f32>, gl_type : gl::types::GLuint ) ->  Vbo{

        let mut vbo : gl::types::GLuint = 0;
        unsafe {

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl_type, vbo);
            gl::BufferData(
                gl_type,
                ( data.len() * std::mem::size_of::<f32>() ) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl_type, 0);
        }

        Vbo{
            handle : vbo,
            gl_type :gl_type
        }
    }

    pub fn get_handle( &self ) -> gl::types::GLuint{
        self.handle
    }

    pub fn get_gl_type(&self) -> gl::types::GLuint {
        self.gl_type
    }

}

impl Drop for Vbo{
    fn drop(&mut self){
        println!("VBO dropped {}", self.handle);
        unsafe{
            gl::DeleteBuffers(1, &mut self.handle);
        }
    }
}
