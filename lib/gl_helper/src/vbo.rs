extern crate gl;

// #[derive(Copy)]
pub struct Vbo{
    handle: gl::types::GLuint,
    gl_type :  gl::types::GLuint,
    number_of_items : usize,
}

impl Vbo{

    pub fn new<T>( data : &Vec<T>, gl_type : gl::types::GLuint ) ->  Vbo{

        let mut vbo : gl::types::GLuint = 0;
        unsafe {

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl_type, vbo);
            gl::BufferData(
                gl_type,
                ( data.len() * std::mem::size_of::<T>() ) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl_type, 0);
        }

        Vbo{
            handle : vbo,
            gl_type :gl_type,
            number_of_items : data.len()
        }
    }

    pub fn get_handle( &self ) -> gl::types::GLuint{
        self.handle
    }

    pub fn get_gl_type(&self) -> gl::types::GLuint {
        self.gl_type
    }

    pub fn len(&self) -> usize{
        self.number_of_items
    }
}

impl Drop for Vbo{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1, &mut self.handle);
        }
    }
}
