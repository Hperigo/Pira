use crate::gl_helper::GlslProg;
use crate::gl_helper::Vbo;
use glow;
use glow::HasContext;

use std::collections::hash_map::HashMap;

use super::StockShader;

/*
TODO: add traits to Vertex Attribs and use a templated version of it
    get_data_sclice();
    get_stride();
    get_size();
*/

pub struct VertexAttrib<'a> {
    pub name: &'static str,
    pub size: i32,
    pub stride: i32,
    pub data: &'a [u8],
    pub per_instance: bool, // alias to attrib divisor
}

impl<'a> VertexAttrib<'a> {
    pub fn new<T>(
        name: &'static str,
        number_of_elements_per_component: i32,
        stride: i32,
        data: &Vec<T>,
        per_instance: bool,
    ) -> Self {
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>(),
            )
        };

        VertexAttrib {
            name,
            size: number_of_elements_per_component,
            stride,
            data,
            per_instance,
        }
    }

    pub fn new_position_attr_with_data(data: &Vec<f32>) -> Self {
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            )
        };

        let position_attr = VertexAttrib {
            name: StockShader::attrib_name_position(),
            size: 3,
            stride: 0,
            data,
            per_instance: false,
        };
        position_attr
    }

    pub fn new_color_attr_with_data(data: &Vec<f32>) -> Self {
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            )
        };

        let color_attrib = Self {
            name: StockShader::attrib_name_color(),
            size: 4,
            stride: 0,
            data,
            per_instance: false,
        };

        color_attrib
    }

    pub fn new_texture_attr_with_data(data: &Vec<f32>) -> Self {
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            )
        };

        let texture_attrib = Self {
            name: StockShader::attrib_name_texture_coords(),
            size: 2,
            stride: 0,
            data,
            per_instance: false,
        };

        texture_attrib
    }

    pub fn new_normal_attr_with_data(data: &Vec<f32>) -> Self {
        let data: &[u8] = unsafe {
            core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<f32>(),
            )
        };

        let texture_attrib = Self {
            name: StockShader::attrib_name_normal(),
            size: 3,
            stride: 0,
            data,
            per_instance: false,
        };

        texture_attrib
    }
}

pub struct Vao {
    handle: Option<glow::VertexArray>,
    vbos: HashMap<&'static str, Vbo>,
    num_of_vertices: usize,
    index_buffer: Option<Vbo>,
    draw_mode: u32,
}

impl Vao {
    pub fn new_from_attrib_indexed(
        gl: &glow::Context,
        attribs: &[VertexAttrib],
        indices: &[u32],
        mode: u32,
        shader: &GlslProg,
    ) -> Option<Vao> {
        let mut vao = Self::new_from_attrib(gl, attribs, mode, shader).unwrap();
        let index_vbo = Vbo::new(gl, indices, glow::ELEMENT_ARRAY_BUFFER);

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
        mode: u32,
        shader: &GlslProg,
    ) -> Option<Self> {
        let num_of_vertices =
            attribs[0].data.len() / (attribs[0].size as usize * core::mem::size_of::<f32>());
        let vao_handle = unsafe { gl.create_vertex_array().unwrap() };

        let mut attrib_map: HashMap<&'static str, Vbo> = HashMap::new();

        unsafe {
            gl.bind_vertex_array(Some(vao_handle));
        };

        for i in 0..attribs.len() {
            let attrib = &attribs[i];
            let name = attrib.name;

            unsafe {
                let loc = gl
                    .get_attrib_location(
                        shader
                            .get_handle()
                            .expect("provided GlslProg is NONE, did it compiled properly? "),
                        name,
                    )
                    .expect(format!("unable to find attribute with name: {}", name).as_str());

                gl.enable_vertex_attrib_array(loc);

                let data_vbo =
                    Vbo::new_from_raw_parts(gl, attrib.data, num_of_vertices, glow::ARRAY_BUFFER);
                gl.bind_buffer(data_vbo.get_gl_type(), data_vbo.get_handle());

                gl.vertex_attrib_pointer_f32(
                    loc,
                    attrib.size,
                    glow::FLOAT,
                    false,
                    attrib.stride,
                    0,
                );

                let attrib_divisor: u32 = if attrib.per_instance { 1 } else { 0 };

                gl.vertex_attrib_divisor(loc, attrib_divisor);

                // end
                gl.bind_buffer(data_vbo.get_gl_type(), None);
                attrib_map.insert(name, data_vbo);
            };
        }

        unsafe {
            gl.bind_vertex_array(None);
        }

        // return
        let vao = Self {
            draw_mode: mode,
            handle: Some(vao_handle),
            vbos: attrib_map,
            num_of_vertices,
            index_buffer: None,
        };
        Some(vao)
    }

    pub fn set_draw_mode(&mut self, mode: u32) {
        self.draw_mode = mode;
    }

    pub fn get_draw_mode(&mut self) -> u32 {
        self.draw_mode
    }

    pub fn get_handle(&self) -> Option<glow::VertexArray> {
        self.handle
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
            gl.draw_arrays_instanced(
                self.draw_mode,
                0,
                self.num_of_vertices as i32,
                instance_count,
            );
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
        for (_, val) in &mut self.vbos {
            val.delete(gl);
        }

        unsafe {
            gl.delete_vertex_array(self.handle.unwrap());
        };
        self.handle = None;
    }
}
