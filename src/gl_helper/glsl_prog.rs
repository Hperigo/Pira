
extern crate nalgebra_glm as glm;

use std::ffi::{CString};
use glow::{self, HasContext};
pub struct GlslProg{
    handle: Option<glow::Program>,
}

impl GlslProg{
    pub fn new(gl : &glow::Context, vertex_source : &str, frag_source : &str) -> GlslProg {

        let vertex_handle = compile_shader( gl, vertex_source, glow::VERTEX_SHADER );
        let frag_handle = compile_shader( gl, frag_source,  glow::FRAGMENT_SHADER );

        let program_id = unsafe{ gl.create_program().unwrap() };
        
        unsafe{

            gl.attach_shader(program_id, vertex_handle);
            gl.attach_shader(program_id, frag_handle);
            gl.link_program(program_id);
            let success = gl.get_program_link_status(program_id);

            if success == false{
                gl.get_program_info_log(program_id);
                return Self{
                    handle : None,
                };
            }

            gl.detach_shader(program_id, vertex_handle);
            gl.detach_shader(program_id, frag_handle);
        }

        GlslProg{
            handle : Some(program_id)
        }
    }


    pub fn get_handle(&self) -> Option<glow::Program> {
        return self.handle;
    }

    pub fn set_uniform_mat4(&self, gl : &glow::Context, name : &str, value : &glm::Mat4){

        unsafe{
            let loc = gl.get_uniform_location(self.handle.unwrap(), name).unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&loc), false, value.as_slice());
        };
    }

    pub fn set_uniform_4f(&self, gl : &glow::Context, name : &str, value : &glm::Vec4){

        unsafe{
            let loc = gl.get_uniform_location(self.handle.unwrap(), name).unwrap();
            gl.uniform_4_f32(Some(&loc), value.x, value.y, value.z, value.w);
        };
    }
    /*
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

   
*/

    pub fn bind(&self, gl : &glow::Context){
        unsafe{
            assert!( self.handle != None );
            gl.use_program(self.handle);
        }
    }

    pub fn unbind(&self, gl : &glow::Context){
        unsafe{
            gl.use_program(None);
        }
    }

    pub fn delete(&self, gl : &glow::Context){
        unsafe{
            gl.delete_program(self.handle.unwrap());
        }        
    }
}

fn compile_shader( gl : &glow::Context, src : &str, shader_type : u32 ) -> glow::Shader {

        let shader_id = unsafe { gl.create_shader(shader_type).unwrap() };
        
        unsafe {
            gl.shader_source(shader_id, src);
            gl.compile_shader(shader_id);
        }

        let success = unsafe{ gl.get_shader_compile_status(shader_id)};
        if success == false
        {   
            let shader_type_string : &str;            
            match shader_type {
                glow::VERTEX_SHADER => shader_type_string = "VERTEX_SHADER",
                glow::FRAGMENT_SHADER => shader_type_string = "FRAGMENT",
                _ => shader_type_string = "unkwon shader type"
            };
            unsafe {
                let log = gl.get_shader_info_log(shader_id);
                println!("Failed to compile {} :: error {}", shader_type_string, log );
            }
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