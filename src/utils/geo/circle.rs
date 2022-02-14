use super::*;
use crate::gl_helper as glh;

pub struct Circle{ 
   pub data : GeometryData,
   pub radius : f32,
   pub x : f32,
   pub y : f32,
}

impl Circle {
    pub fn new(x : f32, y : f32, radius : f32 ) -> Circle {
        /*
            [0]--------[1]
             |			|
             |			|
            [2]--------[3]
        */

        let mut vertices: Vec<f32> = Vec::new();
        vertices.append(&mut vec![x, y, 0.0]);

        for i in 0..33 {
            let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
            let xx = angle.cos() * radius;
            let yy = angle.sin() * radius;

            vertices.append(&mut vec![xx + x, yy + y, 0.0]);
        }


        let number_of_vertices = vertices.len();
        let attrib = glh::VertexAttrib::new_position_attr_with_data(vertices);

        let mut attribs = HashMap::new();
        attribs.insert( glh::StockShader::attrib_name_position().to_string(), attrib );

        Circle{
            data : GeometryData{
                number_of_vertices,
                attribs : attribs,
                ..Default::default()
            },
            radius,
            x,y
        }
    }

    pub fn texture_coords<'a>(&'a mut self) -> &'a mut Self {

        let data = GeometryData::gen_func(self, self.data.number_of_vertices, 2, | circle, color_vertices , index|{
            let vertices = &circle.data.attribs.get( &glh::StockShader::attrib_name_position().to_string() ).unwrap().data;
                let color_index =  index * 4;
                let position_index = index * 3;
                color_vertices[color_index] = (vertices[position_index] - circle.x) / (circle.radius * 2.0);
                color_vertices[color_index + 1] = (vertices[position_index + 1] - circle.y) / (circle.radius * 2.0);
        });

        self.data.attribs.insert( glh::StockShader::attrib_name_texture_coords().to_string(),  glh::VertexAttrib::new_texture_attr_with_data(data));
        self
    }

    pub fn vertex_color<'a, T>(&'a mut self, generator : T )-> &'a mut Self where T : Fn(&Self, &mut Vec<f32>, usize) {
       let data = GeometryData::gen_func(self, self.data.number_of_vertices, 4, generator);
       self.data.attribs.insert( glh::StockShader::attrib_name_color().to_string(),  glh::VertexAttrib::new_color_attr_with_data(data) );
       self
    }
}

impl Geometry for Circle {
    fn get_vertex_attribs(&mut self) -> Vec<glh::VertexAttrib> {
        super::collect_vertex_attribs(&mut self.data.attribs)
    }

    
    fn get_vao_and_shader(&mut self, gl : &glow::Context) -> (glh::Vao, glh::GlslProg){
        gen_vao_and_shader(gl, glow::TRIANGLE_FAN, None, &mut self.data.attribs)
    }
}
