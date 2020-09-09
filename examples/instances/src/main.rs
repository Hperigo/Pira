extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use piralib::glm;

use std::ffi::CString;

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        title: "#️⃣".to_string()
    });

    let vertex_shader_string = "#version 330

    uniform mat4 uModelMatrix;
    uniform mat4 uPerspectiveMatrix;
    uniform mat4 uViewMatrix;

    layout (location = 0) in vec3 inPosition;
    layout (location = 1) in vec4 inColor;
   
    layout (location = 2) in vec3 instancePosition;

    out vec4 vertexColor;

    void main()
    {
        gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * vec4(inPosition + instancePosition, 1.0);
        vertexColor = inColor;
    }
    ";

    let frag_shader_string = "#version 330
    uniform vec4 uColor;
    in vec4 vertexColor; //in vec4 vertexColor;

    out vec4 Color;
    void main()
    {
        Color = uColor * vertexColor;
    }
    ";

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();

    // build vertex data ----
    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![100.0, 130.0, 0.0] );
    vertices.append( &mut vec![0.0,   130.0, 0.0,] );

    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![100.0, 130.0, 0.0] );
    vertices.append( &mut vec![100.0, 0.0, 0.0] );

    let mut colors : Vec<f32> = Vec::new();
    {   
        let num_of_vertices = vertices.len();
        let mut i = 0;
        while i < num_of_vertices {
            let red : f32 = i as f32 / num_of_vertices as f32;
            colors.append(&mut vec![red, 0.0, 0.0, 1.0]);
            i = i + 3;
        }
    } 

    let instance_positions = vec![0.0, 0.0, 0.0, 
                                  100.0, 100.0, 0.0,
                                  300.0, 300.0, 0.0];


    let instance_positions_attrib = glh::VertexAttrib{
        name : "instancePosition",
        size : 3,
        stride : 0, 
        data : instance_positions,
        per_instance : true,
    };

    pos_attrib.data = vertices;
    color_attrib.data = colors;

    let shader = glh::GlslProg::new(&CString::new(vertex_shader_string).unwrap(), &CString::new(frag_shader_string).unwrap());
    
    let attribs = vec![pos_attrib, color_attrib, instance_positions_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader).unwrap();

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
    }

    while app.run() {

        glh::clear(0.2, 0.1, 0.1, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32 * 0.5,
                                    app.get_framebuffer_size().1 as f32 * 0.5,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw_instanced( gl::TRIANGLES, 3 );

        shader.unbind();
    }
}
