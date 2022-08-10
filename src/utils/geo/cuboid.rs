use super::*;
use crate::gl_helper::{self as glh};

pub struct Cuboid {
    pub data: GeometryData,
    pub width: f32,
    pub height: f32,
    pub length: f32,
}

impl Cuboid {
    pub fn new_with_uniform_size(size: f32) -> Self {
        let mut vertices = vec![
            1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        ];

        for pos in &mut vertices {
            *pos = *pos * size;
        }

        let number_of_vertices = vertices.len();
        //let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let indices = vec![
            1, 14, 20, 1, 20, 7, 10, 6, 19, 10, 19, 23, 21, 18, 12, 21, 12, 15, 16, 3, 9, 16, 9,
            22, 5, 2, 8, 5, 8, 11, 17, 13, 0, 17, 0, 4,
        ];
        let mut attribs = HashMap::new();

        attribs.insert(
            glh::StockShader::attrib_name_position().to_string(),
            vertices,
        );

        Cuboid {
            data: GeometryData {
                number_of_vertices,
                attribs: attribs,
                indices: Some(indices),
                ..Default::default()
            },
            width: size,
            height: size,
            length: size,
        }
    }

    pub fn texture_coords<'a>(&'a mut self) -> &'a mut Self {
        let data = vec![
            0.625, 0.5, 0.625, 0.5, 0.625, 0.5, 0.375, 0.5, 0.375, 0.5, 0.375, 0.5, 0.625, 0.25,
            0.625, 0.25, 0.625, 0.25, 0.375, 0.25, 0.375, 0.25, 0.375, 0.25, 0.625, 0.75, 0.625,
            0.75, 0.875, 0.5, 0.375, 0.75, 0.125, 0.5, 0.375, 0.75, 0.625, 1.0, 0.625, 0.0, 0.875,
            0.25, 0.375, 1.0, 0.125, 0.25, 0.375, 0.0,
        ];

        self.data.attribs.insert(
            glh::StockShader::attrib_name_texture_coords().to_string(),
            data,
        );
        self
    }

    pub fn normals<'a>(&'a mut self) -> &'a mut Self {
        let data = vec![
            0.0, 0.0, -1.0, 0.0, 1.0, -0.0, 1.0, 0.0, -0.0, 0.0, -1.0, -0.0, 0.0, 0.0, -1.0, 1.0,
            0.0, -0.0, 0.0, 0.0, 1.0, 0.0, 1.0, -0.0, 1.0, 0.0, -0.0, 0.0, -1.0, -0.0, 0.0, 0.0,
            1.0, 1.0, 0.0, -0.0, -1.0, 0.0, -0.0, 0.0, 0.0, -1.0, 0.0, 1.0, -0.0, -1.0, 0.0, -0.0,
            0.0, -1.0, -0.0, 0.0, 0.0, -1.0, -1.0, 0.0, -0.0, 0.0, 0.0, 1.0, 0.0, 1.0, -0.0, -1.0,
            0.0, -0.0, 0.0, -1.0, -0.0, 0.0, 0.0, 1.0,
        ];

        self.data
            .attribs
            .insert(glh::StockShader::attrib_name_normal().to_string(), data);
        self
    }
}

impl Geometry for Cuboid {
    fn get_vao_and_shader(&mut self, gl: &glow::Context) -> (glh::Vao, glh::GlslProg) {
        gen_vao_and_shader(
            gl,
            glow::TRIANGLES,
            &mut self.data.attribs,
            self.data.indices.as_ref(),
        )
    }

    fn get_vao(&mut self, gl: &glow::Context, glsl_prog: &glh::GlslProg) -> glh::Vao {
        gen_vao(
            gl,
            glow::TRIANGLES,
            &mut self.data.attribs,
            self.data.indices.as_ref(),
            glsl_prog,
        )
    }
}
