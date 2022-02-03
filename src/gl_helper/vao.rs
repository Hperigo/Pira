use crate::gl_helper::GlslProg;
use crate::gl_helper::StockShader;
use crate::gl_helper::Vbo;
use glow;
use glow::HasContext;


/*
TODO: add traits to Vertex Attribs and use a templated version of it
    get_data_sclice();
    get_stride();
    get_size();
*/
#[derive(Debug)]
pub struct VertexAttrib {
    pub name: &'static str,
    pub size: i32,
    pub stride: i32,
    pub data: Vec<f32>,
    pub per_instance: bool, // alias to attrib divisor

}

impl VertexAttrib {
    pub fn new_position_attr() -> VertexAttrib {
        let position_attr = VertexAttrib {
            name: StockShader::attrib_name_position(),
            size: 3,
            stride: 0,
            data: Vec::<f32>::new(),
            per_instance: false,
        };

        position_attr
    }

    pub fn new_color_attr() -> VertexAttrib {
        let color_attrib = VertexAttrib {
            name: StockShader::attrib_name_color(),
            size: 4,
            stride: 0,
            data: Vec::<f32>::new(),
            per_instance: false,
        };

        color_attrib
    }

    pub fn new_texture_attr() -> VertexAttrib {
        let texture_attrib = VertexAttrib {
            name: StockShader::attrib_name_texture_coords(),
            size: 2,
            stride: 0,
            data: Vec::<f32>::new(),
            per_instance: false,
        };

        texture_attrib
    }
}

pub struct Vao {
    handle: Option<glow::VertexArray>,
    vbo_handle: Vbo,
    num_of_vertices: usize,
    index_buffer: Option<Vbo>,
    draw_mode : u32,
}

impl Vao {
    pub fn new_from_attrib_indexed(
        gl: &glow::Context,
        attribs: &[VertexAttrib],
        indices: &[u32],
        mode : u32,
        shader: &GlslProg,
    ) -> Option<Vao> {
        let mut vao = Vao::new_from_attrib(gl, attribs, mode, shader).unwrap();
        let index_vbo = Vbo::new(gl,indices, glow::ELEMENT_ARRAY_BUFFER);

        vao.bind(gl);
        index_vbo.bind(gl);

        vao.unbind(gl);
        index_vbo.unbind(gl);

        vao.index_buffer = Some(index_vbo);
        Some(vao)
    }

    pub fn new_from_attrib(
        gl: &glow::Context,
        attribs: &[VertexAttrib],
        mode : u32,
        shader: &GlslProg,
    ) -> Option<Vao> {
        let mut data = Vec::<f32>::new();

        // merge buffers
        // TODO: we dont need to flatten the data into a single array, a better aproach would be to just buffer with an offset
        for a in attribs {
            if a.data.len() > 0 {
                data.append(&mut a.data.clone());
            }
            
        }

        let num_of_vertices = attribs[0].data.len() / attribs[0].size as usize;
        let vao_handle = unsafe { gl.create_vertex_array().unwrap() };
        let data_vbo = Vbo::new(gl, &data, glow::ARRAY_BUFFER);

        unsafe {
            gl.bind_vertex_array(Some(vao_handle));
            gl.bind_buffer(data_vbo.get_gl_type(), data_vbo.get_handle());
            
            let mut current_offset: usize = 0;
            for a in attribs {
                if a.data.len() == 0 {
                    continue;
                }
                let name = a.name;
                println!("name: {}", name);
                let loc = gl
                    .get_attrib_location(
                        shader
                            .get_handle()
                            .expect("provided shader for attrib is None!"),
                        name,
                    )
                    .expect(format!("unable to find attribute with name: {}", name).as_str());

                let loc = loc;
                gl.enable_vertex_attrib_array(loc);
                gl.vertex_attrib_pointer_f32(
                    loc,
                    a.size,
                    glow::FLOAT,
                    false,
                    a.stride,
                    current_offset as i32,
                );

                let attrib_divisor: u32 = if a.per_instance { 1 } else { 0 };

                gl.vertex_attrib_divisor(loc, attrib_divisor);
                current_offset += a.data.len() * std::mem::size_of::<f32>();
            }

            data_vbo.unbind(gl);
            gl.bind_vertex_array(None);
        }

        // return
        let vao = Vao {
            draw_mode : mode,
            handle: Some(vao_handle),
            vbo_handle: data_vbo,
            num_of_vertices,
            index_buffer: None,
        };

        Some(vao)
    }

    pub fn set_draw_mode(&mut self, mode : u32 ){
        self.draw_mode = mode;
    }

    pub fn get_draw_mode(&mut self) -> u32 {
        self.draw_mode
    }

    pub fn get_handle(&self) -> Option<glow::VertexArray> {
        self.handle
    }

    pub fn buffer_sub_data(&self, gl: &glow::Context, data: &[f32], size: i32) {
        unsafe {
            gl.bind_vertex_array(self.get_handle());
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo_handle.get_handle());

            let data_u8: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            );

            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, data_u8);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                size,
                glow::FLOAT,
                false,
                3 * std::mem::size_of::<f32>() as i32,
                0,
            );

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            // gl::BindVertexArray(self.handle);
            gl.bind_vertex_array(self.handle);
        }
    }

    pub fn unbind(&self, gl: &glow::Context) {
        unsafe {
            // gl::BindVertexArray(0);
            gl.bind_vertex_array(None);
        }
    }

    pub fn draw_instanced(&self, gl: &glow::Context, instance_count: i32) {
        unsafe {
            self.bind(gl);
            //gl::DrawArraysInstanced(primitive, 0, self.num_of_vertices as i32, instance_count);
            gl.draw_arrays_instanced(self.draw_mode, 0, self.num_of_vertices as i32, instance_count);
            self.unbind(gl);
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        self.bind(gl);
        match &self.index_buffer {
            Some(element_buffer) => unsafe {
                gl.draw_elements(
                    self.draw_mode,
                    element_buffer.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            },
            None => unsafe {
                gl.draw_arrays(self.draw_mode, 0, self.num_of_vertices as i32);
            },
        }
        self.unbind(gl);
    }

    pub fn delete(&mut self, gl: &glow::Context) {

        self.vbo_handle.delete(gl);
        unsafe { 
            gl.delete_vertex_array(self.handle.unwrap());
        };
        self.handle = None;
    }
}
