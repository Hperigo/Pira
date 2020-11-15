
extern crate nalgebra_glm as glm;

use crate::gl;
use std::ffi::{CString, CStr};

pub struct GlslProg{
    handle: gl::types::GLuint,
}

impl GlslProg{
    pub fn new(vertex_source : &CStr, frag_source : &CStr) -> GlslProg {

        let vertex_handle = compile_shader( vertex_source, gl::VERTEX_SHADER );
        let frag_handle = compile_shader( frag_source, gl::FRAGMENT_SHADER );

        let program_id = unsafe{ gl::CreateProgram() };
        let mut success : gl::types::GLint = 1;

        unsafe{

            gl::AttachShader(program_id, vertex_handle);
            gl::AttachShader(program_id, frag_handle);
            gl::LinkProgram(program_id);

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0{

            let mut len : gl::types::GLint = 0;

            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len( len as usize );

                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
                println!("link shader error: {}", error.into_string().unwrap());

            }

            return GlslProg{
                handle : 0
            };

        }


        unsafe{
            gl::DetachShader(program_id, vertex_handle);
            gl::DetachShader(program_id, frag_handle);
        }

        GlslProg{
            handle : program_id
        }
    }


    pub fn get_handle(&self) -> u32 {
        return self.handle;
    }

    pub fn set_uniform_1i(&self, name : &str, value : &i32){
        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            gl::Uniform1i( loc, *value );
        };
    }
    
    pub fn set_uniform_1f(&self, name : &str, value : f32){
        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            gl::Uniform1f( loc, value );
        };
    }

     pub fn set_uniform_2f(&self, name : &str, value : &[f32; 2]){
        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            gl::Uniform2f( loc, value[0], value[1] );
        };
    }

   pub fn set_uniform_3f(&self, name : &str, value : &[f32; 3]){
        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            gl::Uniform3f( loc, value[0], value[1], value[2] );
        };
    }

    pub fn set_uniform_mat4(&self, name : &str, value : &glm::Mat4){
        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            gl::UniformMatrix4fv( loc, 1, gl::FALSE, value.as_slice().as_ptr() as *const f32 );
        };
    }

    pub fn set_uniform_4f(&self, name : &str, value : &glm::Vec4){

        let cname = CString::new( name ).expect("ill formed string");
        unsafe{
            let loc = gl::GetUniformLocation(self.handle,  cname.as_bytes_with_nul().as_ptr() as *const i8 );
            // gl::UniformMatrix4fv( loc, 1, gl::FALSE, value.as_slice().as_ptr() as *const f32 );
            gl::Uniform4fv(loc, 1, value.as_slice().as_ptr() as * const f32);
        };
    }


    pub fn bind(&self){
        unsafe{
            gl::UseProgram(self.handle);
        }
    }

    pub fn unbind(&self){
        unsafe{
            gl::UseProgram(0);
        }
    }
}

impl Drop for GlslProg{
    fn drop(&mut self){
        unsafe{
            gl::DeleteProgram( self.handle );
        }
    }
}


fn compile_shader( src : &CStr, shader_type : gl::types::GLuint ) -> gl::types::GLuint {

        let shader_id = unsafe { gl::CreateShader( shader_type ) };
        unsafe {
            gl::ShaderSource(shader_id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0
        {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                shader_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
                );
            }
            
            let shader_type_string : &str;
            
            match shader_type {
                gl::VERTEX_SHADER => shader_type_string = "VERTEX_SHADER",
                gl::FRAGMENT_SHADER => shader_type_string = "FRAGMENT",
                _ => shader_type_string = "unkwon shader type"
            };

            println!("Failed to compile {} :: error {}", shader_type_string, error.into_string().unwrap() );
        }

        shader_id
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
