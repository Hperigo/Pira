use super::*;
use crate::gl_helper as glh;

pub struct Cuboid{ 
   pub data : GeometryData,
   pub width : f32,
   pub height : f32,
   pub length : f32,
}


impl Cuboid {

    pub fn new_with_uniform_size( size : f32 ) -> Self {
        
        #[rustfmt::skip]
        let mut vertices: Vec<f32> = vec![
            -1.0, -1.0, -1.0,  
             1.0, -1.0, -1.0,  
             1.0,  1.0, -1.0,  
             1.0,  1.0, -1.0,  
            -1.0,  1.0, -1.0,  
            -1.0, -1.0, -1.0,  
    
            -1.0, -1.0,  1.0,  
             1.0, -1.0,  1.0,  
             1.0,  1.0,  1.0,  
             1.0,  1.0,  1.0,  
            -1.0,  1.0,  1.0,  
            -1.0, -1.0,  1.0,  
    
            -1.0,  1.0,  1.0,  
            -1.0,  1.0, -1.0,  
            -1.0, -1.0, -1.0,  
            -1.0, -1.0, -1.0,  
            -1.0, -1.0,  1.0,  
            -1.0,  1.0,  1.0,  
    
             1.0,  1.0,  1.0,  
             1.0,  1.0, -1.0,  
             1.0, -1.0, -1.0,  
             1.0, -1.0, -1.0,  
             1.0, -1.0,  1.0,  
             1.0,  1.0,  1.0,  
    
            -1.0, -1.0, -1.0,  
             1.0, -1.0, -1.0,  
             1.0, -1.0,  1.0,  
             1.0, -1.0,  1.0,  
            -1.0, -1.0,  1.0,  
            -1.0, -1.0, -1.0,  
    
            -1.0,  1.0, -1.0,  
             1.0,  1.0, -1.0,  
             1.0,  1.0,  1.0,  
             1.0,  1.0,  1.0,  
            -1.0,  1.0,  1.0,  
            -1.0,  1.0, -1.0
            ];
    
        for pos in &mut vertices{
            *pos = *pos * size;
        }

        let number_of_vertices = vertices.len();
        //let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let mut attribs = HashMap::new();
        attribs.insert( glh::StockShader::attrib_name_position().to_string(), vertices );

        Cuboid{
            data : GeometryData{
                number_of_vertices,
                attribs : attribs,
                ..Default::default()
            },
            width : size,
            height : size,
            length : size,
        }
    }
}

impl Geometry for Cuboid {
    // fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
    //     super::collect_vertex_attribs(&mut self.data.attribs)
    // }

    
    fn get_vao_and_shader(&mut self, gl : &glow::Context) -> (glh::Vao, glh::GlslProg){
        gen_vao_and_shader(gl, glow::TRIANGLES, &mut self.data.attribs, None)
    }

    // fn get_vao(&mut self, gl : &glow::Context, glsl_prog : &glh::GlslProg) -> glh::Vao {
    //     glh::Vao::new_from_attrib(gl, &self.get_vertex_attribs(),glow::TRIANGLES, glsl_prog).unwrap()
    // }
}
