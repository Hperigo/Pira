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
    uniform float uTime;
    layout (location = 0) in vec3 inPosition;
    layout (location = 1) in vec3 instancePosition;

    void main()
    {
        float angle = uTime * 0.01 + float(gl_InstanceID) * 0.001;
        mat4  rotation = mat4(
            vec4( cos(angle), -sin(angle), 0.0,  0.0 ),
            vec4( sin(angle), cos(angle),  0.0,  0.0 ),
            vec4( 0.0,        0.0,         1.0,  0.0 ),
            vec4( 0.0,        0.0,         0.0,  1.0 ) ); 
        
        vec4 rotatedPoint =  rotation * vec4(inPosition, 0.0) + vec4(instancePosition, 0.0);

        rotatedPoint.w = 1.0;

        gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * rotatedPoint;
        // gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * vec4(inPosition + instancePosition, 1.0); 
    }
    ";

    let frag_shader_string = "#version 330
    uniform vec4 uColor;

    out vec4 Color;
    void main()
    {
        Color = uColor;
    }
    ";

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();

    // build vertex data ----
    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![10.0, 30.0, 0.0] );
    vertices.append( &mut vec![0.0,  30.0, 0.0,] );

    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![10.0, 30.0, 0.0] );
    vertices.append( &mut vec![10.0, 0.0, 0.0] );

    let mut instance_positions : Vec<f32> = Vec::new();
    let spacing = 40.0;
    for x in 0..100{
        for y in 0 .. 100{
            instance_positions.append( &mut vec![x as f32 * spacing * 0.5, y as f32 * spacing, 0.0 ]);
        }
    }


    let number_of_instances = instance_positions.len() as i32;
    println!("number of instances: {}", number_of_instances);

    let instance_positions_attrib = glh::VertexAttrib{
        name : "instancePosition",
        size : 3,
        stride : 0, 
        data : instance_positions,
        per_instance : true,
    };

    pos_attrib.data = vertices;

    let shader = glh::GlslProg::new(&CString::new(vertex_shader_string).unwrap(), &CString::new(frag_shader_string).unwrap());
    let attribs = vec![pos_attrib, instance_positions_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader).unwrap();

    let mut frame_number = 0;
    while app.run() {
        frame_number = frame_number + 1;
        glh::clear(0.2, 0.1, 0.1, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32 * 0.5, // beacuse of mac dpi we need to scale it down
                                    app.get_framebuffer_size().1 as f32 * 0.5,
                                    0.0, -1.0,
                                    1.0));

       shader.set_uniform_1f("uTime", frame_number as f32);

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 100.0, 100.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw_instanced( gl::TRIANGLES, number_of_instances );

        shader.unbind();
    }
}
