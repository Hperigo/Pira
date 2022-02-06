use std::collections::HashMap;
use crate::gl_helper as glh;
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

#[derive(Default)]
pub struct GeometryData{
    pub indices : Option<Vec<u32>>,
    pub attribs : HashMap<String, glh::VertexAttrib>,
    number_of_vertices : usize,
}

impl GeometryData {
    fn gen_func<'a, T, F>( geometry : &'a T, num_of_vertices : usize, stride : usize, generator : F ) -> Vec<f32> where F : Fn(&'a T, &mut Vec<f32>, usize) {
       let data = {
           let mut new_data = vec![0.0; num_of_vertices * stride];
           let mut i = 0;
           let mut vertex_index = 0;
           while i < num_of_vertices {
                generator(geometry, &mut new_data, vertex_index );
                vertex_index += 1;
                i += 3;
           }
           new_data
        };
        data
    }
}

pub trait Geometry {
    fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib>;
}

fn collect_vertex_attribs( attribs_map : &mut HashMap<String, glh::VertexAttrib> ) -> Vec<glh::VertexAttrib> {
    //Note: we do this so we dont actually clone the data from the attrib hashmap, this would be easier if Vao also accepeted a hash map...
    let mut attribs = Vec::new();
    let keys : Vec<String> = attribs_map.keys().cloned().collect();
    for k in &keys {
        let at = attribs_map.remove(k).unwrap();
        attribs.push(at);
    }
    attribs
}