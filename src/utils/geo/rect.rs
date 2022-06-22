use super::*;
use crate::gl_helper as glh;

pub struct Rect {
    pub data: GeometryData,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        /*
            [0]--------[1]
             |			|
             |			|
            [2]--------[3]
        */

        let mut vertices: Vec<f32> = Vec::new();
        vertices.append(&mut vec![x, y, 0.0]); //0
        vertices.append(&mut vec![x + width, y + height, 0.0]); //3
        vertices.append(&mut vec![x, y + height, 0.0]); //2

        vertices.append(&mut vec![x, y, 0.0]); //0
        vertices.append(&mut vec![x + width, y + height, 0.0]); //3
        vertices.append(&mut vec![x + width, y, 0.0]); //1

        let number_of_vertices = vertices.len();
        //let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let mut attribs = HashMap::new();
        attribs.insert(
            glh::StockShader::attrib_name_position().to_string(),
            vertices,
        );

        Rect {
            data: GeometryData {
                number_of_vertices,
                attribs: attribs,
                ..Default::default()
            },
            x,
            y,
            width,
            height,
        }
    }

    pub fn texture_coords<'a>(&'a mut self) -> &'a mut Self {
        let data = GeometryData::gen_func(
            self,
            self.data.number_of_vertices,
            2,
            |rect, color_vertices, index| {
                let vertices = &rect
                    .data
                    .attribs
                    .get(&glh::StockShader::attrib_name_position().to_string())
                    .unwrap();
                let color_index = index * 2;
                let position_index = index * 3;
                color_vertices[color_index] =
                    (vertices[position_index] - rect.x) / rect.width as f32;
                color_vertices[color_index + 1] =
                    (vertices[position_index + 1] - rect.y) / rect.height as f32;
            },
        );

        self.data.attribs.insert(
            glh::StockShader::attrib_name_texture_coords().to_string(),
            data,
        );
        self
    }

    pub fn vertex_color<'a, T>(&'a mut self, generator: T) -> &'a mut Self
    where
        T: Fn(&Self, &mut Vec<f32>, usize),
    {
        let data = GeometryData::gen_func(self, self.data.number_of_vertices, 4, generator);
        self.data
            .attribs
            .insert(glh::StockShader::attrib_name_color().to_string(), data);
        self
    }
}

impl Geometry for Rect {
    // fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
    //     super::collect_vertex_attribs(&mut self.data.attribs)
    // }

    fn get_vao_and_shader(&mut self, gl: &glow::Context) -> (glh::Vao, glh::GlslProg) {
        gen_vao_and_shader(gl, glow::TRIANGLES, &mut self.data.attribs, None)
    }

    // fn get_vao(&mut self, gl: &glow::Context, glsl_prog: &glh::GlslProg) -> glh::Vao {
    //     //glh::Vao::new_from_attrib(gl, &self.get_vertex_attribs(), glow::LINES, glsl_prog).unwrap()
    // }
}
