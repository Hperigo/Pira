
use super::*;
use crate::gl_helper as glh;

pub struct Axis{ 
   pub data : GeometryData,
   pub size : f32,
}


impl Axis {
    pub fn new(size : f32) -> Self {

        let mut vertices: Vec<f32> = Vec::new();

        // X axis 
        vertices.append( &mut vec![0.0, 0.0, 0.0] );
        vertices.append( &mut vec![size, 0.0, 0.0] );

        // Y axis
        vertices.append( &mut vec![0.0, 0.0, 0.0] );
        vertices.append( &mut vec![0.0, size, 0.0] );
 
        // Z axis
        vertices.append( &mut vec![0.0, 0.0, 0.0] );
        vertices.append( &mut vec![0.0, 0.0, size] );

        let number_of_vertices = vertices.len();
        let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let mut attribs = HashMap::new();
        attribs.insert( glh::StockShader::attrib_name_position().to_string(), attrib );

        Self{
            data : GeometryData{
                number_of_vertices,
                attribs : attribs,
                ..Default::default()
            },
            size,
        }
    }

    pub fn vertex_color(&mut self)-> &mut Self {
        let mut data =  Vec::new();
        data.append( &mut vec![1.0, 0.0, 0.0, 1.0] );
        data.append( &mut vec![1.0, 0.0, 0.0, 1.0] );

        data.append( &mut vec![0.0, 1.0, 0.0, 1.0] );
        data.append( &mut vec![0.0, 1.0, 0.0, 1.0] );

        data.append( &mut vec![0.0, 0.0, 1.0, 1.0] );
        data.append( &mut vec![0.0, 0.0, 1.0, 1.0] );
        self.data.attribs.insert( glh::StockShader::attrib_name_color().to_string(),  glh::VertexAttrib::new_color_attr_with_data(data) );
        self
     }
 
     pub fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
         //Note: we do this so we dont actually clone the data from the attrib hashmap, this would be easier if Vao also accepeted a hash map...
         let mut attribs = Vec::new();
         let keys : Vec<String> = self.data.attribs.keys().cloned().collect();
         for k in &keys {
             let at = self.data.attribs.remove(k).unwrap();
             attribs.push(at);
         }
         attribs
     }
}

impl Geometry for Axis {
    fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
        super::collect_vertex_attribs(&mut self.data.attribs)
    }

    fn get_vao_and_shader(&mut self, gl : &glow::Context) -> (glh::Vao, glh::GlslProg){
        gen_vao_and_shader(gl, glow::LINES, None, &mut self.data.attribs)
    }
}

