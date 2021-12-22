use crate::gl_helper as glh;
use crate::gl_helper::VertexAttrib;

pub struct Geometry {
    pub indices: Vec<u32>,
    pub attribs: Vec<VertexAttrib>,
}

impl Geometry {
    pub fn circle(x: f32, y: f32, radius: f32) -> Geometry {
        let mut pos_attrib = glh::VertexAttrib::new_position_attr();

        // build vertex data ----
        let mut vertices: Vec<f32> = Vec::new();
        vertices.append(&mut vec![x, y, 0.0]);

        for i in 0..33 {
            let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
            let xx = angle.cos() * radius;
            let yy = angle.sin() * radius;

            vertices.append(&mut vec![xx + x, yy + y, 0.0]);
        }

        pos_attrib.data = vertices;
        let attribs = vec![pos_attrib];

        Geometry {
            attribs,
            indices: Vec::new(),
        }
    }

    pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Geometry {
        let mut pos_attrib = glh::VertexAttrib::new_position_attr();
        let mut color_attrib = glh::VertexAttrib::new_color_attr();
        let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

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
        vertices.append(&mut vec![x + width, 0.0, 0.0]); //1

        let mut colors: Vec<f32> = Vec::new();
        let mut texure_vertices: Vec<f32> = Vec::new();
        {
            let num_of_vertices = vertices.len();
            let mut i = 0;

            while i < num_of_vertices {
                colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
                texure_vertices.append(&mut vec![
                    vertices[i] / width as f32,
                    vertices[i + 1] / height as f32,
                ]); // normalize vertex coords
                i += 3;
            }
        }

        pos_attrib.data = vertices;
        color_attrib.data = colors;
        texture_attrib.data = texure_vertices;

        Geometry {
            attribs: vec![pos_attrib, color_attrib],
            indices: Vec::new(),
        }
    }
}
