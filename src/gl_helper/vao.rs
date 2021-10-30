use glow;
use glow::HasContext;
use crate::gl_helper::Vbo;
use crate::gl_helper::GlslProg;

use crate::gl_helper::StockShader;
use std::ffi::CString;

#[derive(Debug)]
pub struct VertexAttrib{
    pub name : &'static str,
    pub size : i32,
    pub stride : i32,
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
    handle : Option<glow::VertexArray>,
    vbo_handle : Vbo,
    num_of_vertices : usize,
    index_buffer : Option<Vbo>,
}

impl Vao{
    pub fn new_from_attrib_indexed( gl : &glow::Context, attribs : &Vec<VertexAttrib>, indices : &Vec<u32>, shader : &GlslProg ) -> Option<Vao>{

        let mut vao = Vao::new_from_attrib(gl, attribs, shader).unwrap();
        let index_vbo = Vbo::new(gl, &indices, glow::ELEMENT_ARRAY_BUFFER );

        vao.bind(gl);
        index_vbo.bind(gl);
        
        vao.unbind(gl);
        index_vbo.unbind(gl);

        vao.index_buffer = Some(index_vbo);
        Some(vao)
    }

    pub fn new_from_attrib( gl : &glow::Context, attribs : & Vec<VertexAttrib>, shader : &GlslProg ) -> Option<Vao>{

        let mut data = Vec::<f32>::new();
        // merge buffers
        // TODO: we dont need to flatten the data into a single array, a better aproach would be to just buffer with an offset
        for a in  attribs{
            data.append( &mut a.data.clone() );
        }

        let num_of_vertices = attribs[0].data.len() / attribs[0].size as usize;        
        let mut vao_handle = unsafe {gl.create_vertex_array().unwrap()};
        let data_vbo = Vbo::new(gl, &data, glow::ARRAY_BUFFER );

        unsafe{

            // gl::GenVertexArrays(1, &mut vao_handle);
            // gl::BindVertexArray(vao_handle);
            // gl::BindBuffer(data_vbo.get_gl_type(), data_vbo.get_handle());

            gl.bind_vertex_array(Some(vao_handle));

            let mut current_offset  : usize = 0;
            for a in attribs{
                let name = a.name;
                let loc =  gl.get_attrib_location(
                    shader.get_handle().unwrap(),
                    name
                    );

                if loc == None {
                    println!("Error attrib not found in shader!\n\t {} name: {}", loc.unwrap(), name);
                    return None;
                }
                let loc = loc.unwrap();
                gl.enable_vertex_attrib_array(loc);
                gl.vertex_attrib_pointer_f32(loc, a.size, glow::FLOAT, false, a.stride, current_offset as i32);
                // gl::EnableVertexAttribArray(loc as u32);
                // gl::VertexAttribPointer(
                //     loc as u32,
                //     a.size,
                //     gl::FLOAT,
                //     gl::FALSE,
                //     a.stride,
                //     current_offset as * const gl::types::GLvoid 
                // );
                let attrib_divisor : u32 = if a.per_instance { 1 } else { 0 };
                // gl::VertexAttribDivisor(loc as u32, attrib_divisor);
                gl.vertex_attrib_divisor(loc, attrib_divisor);
                
                current_offset += a.data.len() * std::mem::size_of::<f32>();
            }

            // gl::BindBuffer(data_vbo.get_gl_type(), 0);
            // gl::BindVertexArray(0);
            data_vbo.unbind(gl);
            gl.bind_vertex_array(None);
        }

        // return
        let vao = Vao{
            handle : Some(vao_handle),
            vbo_handle : data_vbo,
            num_of_vertices : num_of_vertices,
            index_buffer : None
        };

        Some(vao)
    }

    pub fn get_handle(&self) -> Option<glow::VertexArray> {
        self.handle
    }

    pub fn buffer_sub_data(&self, gl : &glow::Context, data : &Vec<f32>, size : i32 ){
        unsafe{
            
            gl.bind_vertex_array(self.get_handle());
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo_handle.get_handle());
            // gl::BindVertexArray( self.get_handle() );
            // gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_handle.get_handle() );

            let data_u8: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            );

            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, &data_u8);
            // gl::BufferSubData(
            //     gl::ARRAY_BUFFER, //type
            //     0, //offset
            //     ( data.len() * std::mem::size_of::<f32>() ) as gl::types::GLsizeiptr, // size of data
            //     data.as_ptr() as *const gl::types::GLvoid // data ptr
            //     );
            
            gl.enable_vertex_attrib_array(0); // ? 
            gl.vertex_attrib_pointer_f32(0, size, glow::FLOAT, false, 3 * std::mem::size_of::<f32>() as i32, 0);
            // gl::EnableVertexAttribArray(0);
            // gl::VertexAttribPointer(0, size, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as i32, 0 as *const gl::types::GLvoid);  
            
            gl.bind_buffer(glow::ARRAY_BUFFER,  None);
            gl.bind_vertex_array(None);
            // gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
            // gl::BindVertexArray( 0 );
        }

    }



    pub fn bind(&self, gl : &glow::Context){
        unsafe{
            // gl::BindVertexArray(self.handle);
            gl.bind_vertex_array(self.handle);
        }
        
    }

    pub fn unbind(&self, gl : &glow::Context){
        unsafe{
            // gl::BindVertexArray(0);
            gl.bind_vertex_array(None);
        }        
    }

    pub fn draw_instanced(&self, gl : &glow::Context, primitive : u32, instance_count : i32 ){
        unsafe{
            self.bind(gl);
            //gl::DrawArraysInstanced(primitive, 0, self.num_of_vertices as i32, instance_count);
            gl.draw_arrays_instanced(primitive, 0, self.num_of_vertices as i32, instance_count);
            self.unbind(gl);
        }
    }

    pub fn draw(&self, gl : &glow::Context, primitive : u32 ){

        self.bind(gl);
        match &self.index_buffer {
            Some(element_buffer) => {
                unsafe{
                    gl.draw_elements(primitive, element_buffer.len() as i32, glow::UNSIGNED_INT, 0);
                }
            }
            None => {
                unsafe{
                    gl.draw_arrays(primitive, 0, self.num_of_vertices as i32);
                }
            }
        }
        self.unbind(gl);
    }

    pub fn delete(&mut self, gl : &glow::Context){
        unsafe{ gl.delete_vertex_array( self.handle.unwrap() )  };
        self.handle = None;
    }
}
