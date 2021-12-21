use crate::gl_helper::glsl_prog::GlslProg;
extern crate nalgebra_glm as glm;

use glow;
use std::string::String;

#[derive(Debug, Clone)]
pub struct StockShader {
    texture: (bool, bool), //first element is if shader has texture and second is for flipping the texture
    color: bool,
}

impl StockShader {
    pub fn new() -> StockShader {
        StockShader {
            texture: (false, false),
            color: false,
        }
    }

    pub fn color(&mut self) -> StockShader {
        self.color = true;
        return self.clone();
    }

    pub fn texture(&mut self, flipped: bool) -> StockShader {
        self.texture = (true, flipped);
        return self.clone();
    }

    pub fn get_vertex_string(&self) -> std::string::String {
        #[cfg(not(target_arch = "wasm32"))]
        let shader_version = "#version 400";

        #[cfg(target_arch = "wasm32")]
        let shader_version = "#version 300 es";

        let mut color_layout = String::from("");
        let mut texture_layout = String::from("");

        let mut color_main = String::from("");
        let mut texture_main = String::from("");

        if self.color {
            color_layout = format!(
                "in vec4 {};\n            out vec4 vColor;",
                StockShader::attrib_name_color()
            )
            .to_string();
            color_main = format!("vColor = {};", StockShader::attrib_name_color()).to_string();
        }

        if self.texture.0 {
            texture_layout = format!(
                "in vec2 {};\n            out vec2 textureCoord;",
                StockShader::attrib_name_texture_coords()
            )
            .to_string();
            texture_main = format!(
                "textureCoord = {};",
                StockShader::attrib_name_texture_coords()
            )
            .to_string();
        }

        let position_main = format!(
            "gl_Position = {} * {} * {} * vec4({}, 1.0);",
            StockShader::uniform_name_perspective_matrix(),
            StockShader::uniform_name_view_matrix(),
            StockShader::uniform_name_model_matrix(),
            StockShader::attrib_name_position()
        )
        .to_string();
        let vertex_shader = format!(
            "{}
        precision mediump float;

        uniform mat4 {};
        uniform mat4 {};
        uniform mat4 {};


        in vec3 inPosition;
        {} // color_layout
        {} // texture_layout

        void main()
        {{  
            {} // color_main
            {} // texture_main
            {} // position_main
        }}
        ",
            shader_version,
            //uniforms
            StockShader::uniform_name_model_matrix(),
            StockShader::uniform_name_perspective_matrix(),
            StockShader::uniform_name_view_matrix(),
            color_layout,
            texture_layout,
            color_main,
            texture_main,
            position_main
        );

        vertex_shader
    }

    pub fn get_frag_string(&self) -> std::string::String {
        #[cfg(not(target_arch = "wasm32"))]
        let shader_version = "#version 400";

        #[cfg(target_arch = "wasm32")]
        let shader_version = "#version 300 es";

        let mut sampler_2d = String::from("");
        let mut main_texture_coord = String::from("");
        let mut main_vertex_color = String::from("");

        if self.texture.0 {
            sampler_2d = format!(
                "uniform sampler2D {};",
                StockShader::uniform_name_texture_sampler0()
            );
            main_texture_coord = {
                if self.texture.1 == false {
                    format!(
                        "texture( {}, textureCoord).rgba *",
                        StockShader::uniform_name_texture_sampler0()
                    )
                } else {
                    format!(
                        "texture( {}, vec2(0.0,1.0) - (textureCoord * vec2(-1,1)) ).rgba *",
                        StockShader::uniform_name_texture_sampler0()
                    )
                }
            }
        }

        if self.color {
            main_vertex_color = format!("{} * ", "vColor");
        }

        let frag_shader = format!(
            "{}
        precision mediump float;
        
        uniform vec4 uColor;

        {} // sampler
        in vec2 textureCoord; // in texture coords
        in vec4 vColor; //in vec4 vertexColor;


        out vec4 Color;
        void main()
        {{
            Color = {} {} uColor;
        }}",
            shader_version, sampler_2d, main_vertex_color, main_texture_coord
        );

        frag_shader
    }

    pub fn build(&self, gl: &glow::Context) -> GlslProg {
        let vertex_string = self.get_vertex_string();
        let frag_string = self.get_frag_string();

        let prog = GlslProg::new(gl, vertex_string.as_str(), frag_string.as_str());

        // set some default values for uniforms
        prog.bind(gl);
        prog.set_uniform_4f(gl, StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0]);
        prog.unbind(gl);

        prog
    }

    // Default uniforms and attribute names ---
    // we can use this query the names of variables used on the stock shader
    pub fn uniform_name_model_matrix() -> &'static str {
        return "uModelMatrix";
    }
    pub fn uniform_name_perspective_matrix() -> &'static str {
        return "uPerspectiveMatrix";
    }
    pub fn uniform_name_view_matrix() -> &'static str {
        return "uViewMatrix";
    }
    pub fn uniform_name_texture_sampler0() -> &'static str {
        return "tex0";
    }
    pub fn uniform_name_color() -> &'static str {
        return "uColor";
    }
    pub fn attrib_name_position() -> &'static str {
        return "inPosition";
    }
    pub fn attrib_name_color() -> &'static str {
        return "inColor";
    }
    pub fn attrib_name_texture_coords() -> &'static str {
        return "inTexture";
    }
}
