use crate::gl_helper::{self as glh, StockShader, VertexAttrib};
use std::collections::HashMap;
// TODO: replace geometry with this struct
// Trait geometry with some standard functions, like:
//     1. color and uv from bounds.
//     2. generator lambda
//

// 2D
pub mod rect;
pub use rect::Rect;

pub mod circle;
pub use circle::Circle;

// 3D
pub mod axis;
pub use axis::Axis;

pub mod cuboid;
pub use cuboid::Cuboid;

pub mod sphere;
pub use sphere::Sphere;

#[derive(Default)]
pub struct GeometryData {
    pub indices: Option<Vec<u32>>,
    pub attribs: HashMap<String, Vec<f32>>,
    number_of_vertices: usize,
}

impl GeometryData {
    fn gen_func<'a, T, F>(
        geometry: &'a T,
        num_of_vertices: usize,
        stride: usize,
        generator: F,
    ) -> Vec<f32>
    where
        F: Fn(&'a T, &mut Vec<f32>, usize),
    {
        let data = {
            let mut new_data = vec![0.0; num_of_vertices * stride];
            let mut i = 0;
            let mut vertex_index = 0;
            while i < num_of_vertices {
                generator(geometry, &mut new_data, vertex_index);
                vertex_index += 1;
                i += 3;
            }
            new_data
        };
        data
    }
}

pub trait Geometry {
    //fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib>;
    fn get_vao_and_shader(&mut self, gl: &glow::Context) -> (glh::Vao, glh::GlslProg);
    fn get_vao(&mut self, gl: &glow::Context, glsl_prog: &glh::GlslProg) -> glh::Vao;
}

fn gen_vao_and_shader(
    gl: &glow::Context,
    mode: u32,
    attribs_map: &mut HashMap<String, Vec<f32>>,
    indices: Option<&Vec<u32>>,
) -> (glh::Vao, glh::GlslProg) {
    let (attribs_vec, shader_factory) = collect_vertex_attribs(attribs_map);
    let shader = shader_factory.build(gl);

    let vao = {
        if indices.is_some() {
            // glh::Vao::new_from_attrib_indexed(gl, &attribs_vec, &indices.unwrap(), mode, &shader)
            //     .unwrap()
            glh::Vao::new_from_attrib_indexed(gl, &attribs_vec, &indices.unwrap(), mode, &shader)
                .unwrap()
        } else {
            glh::Vao::new_from_attrib(gl, &attribs_vec, mode, &shader).unwrap()
        }
    };

    (vao, shader)
}

fn gen_vao(
    gl: &glow::Context,
    mode: u32,
    attribs_map: &mut HashMap<String, Vec<f32>>,
    indices: Option<&Vec<u32>>,
    shader: &glh::GlslProg,
) -> glh::Vao {
    let (attribs_vec, _) = collect_vertex_attribs(attribs_map);

    let vao = {
        if indices.is_some() {
            glh::Vao::new_from_attrib_indexed(gl, &attribs_vec, &indices.unwrap(), mode, &shader)
                .unwrap()
        } else {
            glh::Vao::new_from_attrib(gl, &attribs_vec, mode, &shader).unwrap()
        }
    };

    return vao;
}

fn collect_vertex_attribs(
    attribs_map: &mut HashMap<String, Vec<f32>>,
) -> (Vec<glh::VertexAttrib>, StockShader) {
    let mut shader_factory = glh::StockShader::new();
    let mut attribs_vec = Vec::new();

    // POSITION
    let pos_data = attribs_map
        .get(glh::StockShader::attrib_name_position())
        .unwrap();

    let attrib_pos = VertexAttrib::new_position_attr_with_data(pos_data);
    attribs_vec.push(attrib_pos);

    // VERTEX COLOR
    if attribs_map.contains_key(glh::StockShader::attrib_name_color()) {
        shader_factory.color();

        let color_data = attribs_map
            .get(glh::StockShader::attrib_name_color())
            .unwrap();

        let attrib_color = VertexAttrib::new_color_attr_with_data(color_data);
        attribs_vec.push(attrib_color);
    }

    // TEXTURE COORDS
    if attribs_map.contains_key(glh::StockShader::attrib_name_texture_coords()) {
        shader_factory.texture(true);

        let data = attribs_map
            .get(glh::StockShader::attrib_name_texture_coords())
            .unwrap();

        let attrib = VertexAttrib::new_texture_attr_with_data(data);
        attribs_vec.push(attrib);
    }

    // NORMALS
    if attribs_map.contains_key(glh::StockShader::attrib_name_normal()) {
        let data = attribs_map
            .get(glh::StockShader::attrib_name_normal())
            .unwrap();

        let attrib = VertexAttrib::new_normal_attr_with_data(data);
        attribs_vec.push(attrib);
    }

    (attribs_vec, shader_factory)
}
