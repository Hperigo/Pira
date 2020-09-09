extern crate gl;
use crate::Vbo;
use crate::GlslProg;

use crate::StockShader;

use std::ffi::CString;

#[derive(Debug)]
pub struct VertexAttrib{
    pub name : &'static str,
    pub size : i32,
    pub stride : gl::types::GLint,
    pub data : Vec<f32>,
    pub per_instance : bool, // alias to attrib divisor
}

impl VertexAttrib{

    pub fn new_position_attr() -> VertexAttrib {
        
        let position_attr = VertexAttrib{
            name : StockShader::attrib_name_position(),
            size : 3,
            stride : 0, 
            data : Vec::<f32>::new(),
            per_instance : false,
        };
        
        position_attr
    }


    pub fn new_color_attr() -> VertexAttrib{

        let color_attrib = VertexAttrib{
            name  : StockShader::attrib_name_color(), //String::from("Color"),
            size : 4,
            stride : 0,
            data : Vec::<f32>::new(),
            per_instance : false,
        };

        color_attrib
    }

    pub fn new_texture_attr() -> VertexAttrib{

        let texture_attrib = VertexAttrib{
            name  : StockShader::attrib_name_texture_coords(),
            size : 2,
            stride : 0,
            data : Vec::<f32>::new(),
            per_instance : false,
        };

        texture_attrib
    }
}

pub struct Vao{
    handle : gl::types::GLuint,
    vbo_handle : Vbo,
    num_of_vertices : usize,
}

impl Vao{

    pub fn new_from_attrib( attribs : & Vec<VertexAttrib>, shader : &GlslProg ) -> Option<Vao>{

        let mut data = Vec::<f32>::new();
        // merge buffers
        for a in  attribs{
            data.append( &mut a.data.clone() );
        }

        let num_of_vertices = attribs[0].data.len();

        let mut vao_handle : gl::types::GLuint = 0;
        let data_vbo = Vbo::new(&data, gl::ARRAY_BUFFER );

        unsafe{

            gl::GenVertexArrays(1, &mut vao_handle);
            gl::BindVertexArray(vao_handle);

            gl::BindBuffer(data_vbo.get_gl_type(), data_vbo.get_handle());

            let mut current_offset  : usize = 0;
            for a in attribs{
                let name = &a.name;
                let loc = gl::GetAttribLocation(
                    shader.get_handle(),
                    CString::new( name.as_bytes() ).unwrap().as_ptr()
                    );

                if loc == -1{
                    println!("Error attrib not found in shader!\n\t {} name: {}", loc, name);
                    return None;
                }
                
                gl::EnableVertexAttribArray(loc as u32);
                gl::VertexAttribPointer(
                    loc as u32,
                    a.size,
                    gl::FLOAT,
                    gl::FALSE,
                    a.stride,
                    current_offset as * const gl::types::GLvoid 
                );
                let attrib_divisor : u32 = if a.per_instance { 1 } else { 0 };
                gl::VertexAttribDivisor(loc as u32, attrib_divisor);
                
                current_offset += a.data.len() * std::mem::size_of::<f32>();
            }

            gl::BindBuffer(data_vbo.get_gl_type(), 0);
            gl::BindVertexArray(0);
        }

        // return
        let vao = Vao{
            handle:vao_handle,
            vbo_handle : data_vbo,
            num_of_vertices : num_of_vertices,
        };

        Some(vao)
    }

    pub fn get_handle(&self) -> gl::types::GLuint {
        self.handle
    }

    pub fn buffer_sub_data(&self, data : &Vec<f32>, size : i32 ){
        unsafe{

            gl::BindVertexArray( self.get_handle() );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_handle.get_handle() );

            gl::BufferSubData(
                gl::ARRAY_BUFFER, //type
                0, //offset
                ( data.len() * std::mem::size_of::<f32>() ) as gl::types::GLsizeiptr, // size of data
                data.as_ptr() as *const gl::types::GLvoid // data ptr
                );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, size, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as i32, 0 as *const gl::types::GLvoid);  

            gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
            gl::BindVertexArray( 0 );
        }

    }

    pub fn draw(&self, primitive : gl::types::GLuint ){

        unsafe{

            gl::BindVertexArray( self.get_handle() );
            gl::DrawArrays(
                primitive,
                0,
                self.num_of_vertices as i32
            );
            gl::BindVertexArray( 0 );
        }
    }

    pub fn draw_instanced(&self, primitive : gl::types::GLuint, instance_count : i32 ){
        unsafe{
            gl::BindVertexArray( self.get_handle() );
            gl::DrawArraysInstanced(primitive, 0, self.num_of_vertices as i32, instance_count);
            gl::BindVertexArray( 0 );
        }        
    }


}

impl Drop for Vao{
    fn drop(&mut self){

        unsafe{
            gl::DeleteBuffers(1, &mut self.handle);
        }

    }
}
