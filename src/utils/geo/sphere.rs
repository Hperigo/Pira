use super::*;
use crate::gl_helper as glh;

use std::f32::consts::PI;

pub struct Sphere {
    pub data: GeometryData,
    pub radius: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub rings: u32,
    pub segments: u32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32, rings: u32, segments: u32) -> Self {
        let mut vertices: Vec<f32> = Vec::new();

        let ring_incr = 1.0 / (rings as f32 - 1.0);
        let seg_incr = 1.0 / (segments as f32 - 1.0);

        for r in 0..rings {
            let v = r as f32 * ring_incr;
            for s in 0..segments {
                let u = 1.0 - s as f32 * seg_incr;

                let x = (PI * 2.0 * u).sin() * (PI * v).sin();
                let y = (PI * (v - 0.5)).sin();
                let z = (PI * 2.0 * u).cos() * (PI * v).sin();

                vertices.push(x * radius);
                vertices.push(y * radius);
                vertices.push(z * radius);
            }
        }

        let mut indices: Vec<u32> = Vec::new();
        for r in 0..rings - 1 {
            for s in 0..segments - 1 {
                let index = r * segments + (s + 1);
                indices.push(index);
                let index = r * segments + s;
                indices.push(index);
                let index = (r + 1) * segments + (s + 1);
                indices.push(index);

                let index = (r + 1) * segments + s;
                indices.push(index);
                let index = (r + 1) * segments + (s + 1);
                indices.push(index);
                let index = r * segments + s;
                indices.push(index);
            }
        }

        //
        let number_of_vertices = vertices.len();
        //let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let mut attribs = HashMap::new();
        attribs.insert(
            glh::StockShader::attrib_name_position().to_string(),
            vertices,
        );

        Self {
            data: GeometryData {
                number_of_vertices,
                attribs: attribs,
                indices: Some(indices),
                ..Default::default()
            },
            radius,
            rings,
            segments,
            x,
            y,
            z,
        }
    }

    pub fn texture_coords<'a>(&'a mut self) -> &'a mut Self {
        let ring_incr = 1.0 / (self.rings as f32 - 1.0);
        let seg_incr = 1.0 / (self.segments as f32 - 1.0);

        let mut data = Vec::new();

        for r in 0..self.rings {
            let v = r as f32 * ring_incr;
            for s in 0..self.segments {
                let u = 1.0 - s as f32 * seg_incr;

                data.push(u);
                data.push(v);
            }
        }

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

    pub fn normals<'a>(&'a mut self) -> &'a mut Self {
        let data = GeometryData::gen_func(
            self,
            self.data.number_of_vertices,
            3,
            |sphere, normal_vertices, index| {
                let vertices = &sphere
                    .data
                    .attribs
                    .get(&glh::StockShader::attrib_name_position().to_string())
                    .unwrap();
                let position_index = index * 3;

                let x = (vertices[position_index] - sphere.x) / sphere.radius;
                let y = (vertices[position_index + 1] - sphere.y) / sphere.radius;
                let z = (vertices[position_index + 2] - sphere.z) / sphere.radius;

                normal_vertices[position_index] = x;
                normal_vertices[position_index + 1] = y;
                normal_vertices[position_index + 2] = z;
            },
        );

        self.data
            .attribs
            .insert(glh::StockShader::attrib_name_normal().to_string(), data);
        self
    }
}

impl Geometry for Sphere {
    // fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
    //     super::collect_vertex_attribs(&mut self.data.attribs)
    // }

    fn get_vao_and_shader(&mut self, gl: &glow::Context) -> (glh::VaoSliced, glh::GlslProg) {
        gen_vao_and_shader(
            gl,
            glow::TRIANGLES,
            &mut self.data.attribs,
            self.data.indices.as_ref(),
        )
    }

    // fn get_vao(&mut self, gl : &glow::Context, glsl_prog : &glh::GlslProg) -> glh::Vao {
    //     glh::Vao::new_from_attrib_indexed(gl, &self.get_vertex_attribs(), &self.data.indices.as_ref().unwrap(), glow::TRIANGLES, glsl_prog).unwrap()
    // }
}
