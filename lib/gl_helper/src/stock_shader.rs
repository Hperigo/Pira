use crate::glsl_prog::GlslProg as GlslProg;

use std::ffi::CString;


#[derive(Debug, Clone)]
pub struct StockShader{
    texture : bool,
    color : bool,
}

impl StockShader{

    pub fn new() -> StockShader {
        StockShader{
            texture : false,
            color : false,
        }
    }


    pub fn color(&mut self) -> StockShader {
        self.color = true;
        return self.clone();
    }

    pub fn texture(&mut self) -> StockShader {
        self.texture = true;
        return self.clone();
    }


    pub fn build_vertex_string(&self) -> (StockShader, std::string::String ) {

            let mut color_layout : std::string::String = String::from("");
            let mut texture_layout : std::string::String = String::from("");

            let mut color_main : std::string::String = String::from("");
            let mut texture_main : std::string::String = String::from("");

            if self.color {
                color_layout = std::string::String::from("layout (location = 1) in vec4 Color;\n            out vec4 vertexColor;");
                color_main = std::string::String::from("vertexColor = Color;");
            }

            if self.texture {
                texture_layout = std::string::String::from("layout (location = 2) in vec2 TextureCoords;\n            out vec2 textureCoord;");
                texture_main = std::string::String::from("textureCoord = TextureCoords;");
            }

            let vertex_shader = format!(
            "
            #version 330

            uniform mat4 modelMatrix;
            uniform mat4 perspectiveMatrix;
            uniform mat4 viewMatrix;

            layout (location = 0) in vec3 Position;
            {}
            {}

        
            void main()
            {{
                gl_Position = perspectiveMatrix * viewMatrix * modelMatrix * vec4(Position, 1.0);
                {}
                {}
            }}
            ", color_layout, texture_layout, color_main, texture_main );


            println!("{}",vertex_shader);

            (self.clone(), vertex_shader )
    }

    pub fn build_frag_string(&self) -> (StockShader, std::string::String ) { 

            let mut sampler_2d = "";
            let mut in_texture_coord = "";
            let mut main_texture_coord = "";

            let mut in_vertex_color = "";
            let mut main_vertex_color = "";


            if self.texture {
                sampler_2d = "uniform sampler2D tex0;";
                in_texture_coord = "in vec2 textureCoord;";
                main_texture_coord = "texture( tex0, textureCoord).rgba *";
            }

            if self.color {
                in_vertex_color = "in vec4 vertexColor;";
                main_vertex_color = "vertexColor * ";
            }


            let frag_shader = format!("
            #version 330

            uniform vec4 uColor;

            {} // sampler
            {} // in texture coords

            {} //in vec4 vertexColor;


            out vec4 Color;
            void main()
            {{
                //vec4(textureCoord.x, textureCoord.y, 0, 1.0); //
                Color =   {} {} uColor;
            }}", sampler_2d, in_texture_coord, in_vertex_color, main_vertex_color, main_texture_coord );

            (self.clone(), frag_shader )
    }

    pub fn build(&self) ->  GlslProg {

        let vertex_string = self.build_vertex_string().1;
        let frag_string   = self.build_frag_string().1;

        GlslProg::new( &CString::new(vertex_string).unwrap(), &CString::new(frag_string).unwrap() )
    }

}
