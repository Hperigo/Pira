extern crate piralib;
use piralib::app;
use piralib::gl_helper as glh;
use glow::*;
use nalgebra_glm as glm;
use rand::Rng;

struct FrameData { 
    vao : glh::Vao,
    shader : glh::GlslProg,
    time : f32,
    number_of_instances : i32,
}

fn m_setup( app : &mut app::App) -> FrameData {
    
    let gl = &app.gl;

    let vertex_shader_string = "#version 410

    uniform mat4 uModelMatrix;
    uniform mat4 uPerspectiveMatrix;
    uniform mat4 uViewMatrix;
    uniform float uTime;
    
    uniform vec2 uMousePos;

    layout (location = 0) in vec3 inPosition;
    layout (location = 1) in vec4 inColor; 
    layout (location = 2) in vec2 instancePosition;

    out vec4 vColor;
    void main()
    {   

        float dist = (distance(uMousePos, instancePosition.xy) / 50.0);

        float angle =  float(gl_InstanceID) * 0.0001 + instancePosition.y * 0.007;
        angle = (angle + dist) + uTime * -0.01 ;
        mat3  rotation = mat3(
            vec3( cos(angle), -sin(angle), 0.0),
            vec3( sin(angle), cos(angle),  0.0),
            vec3( 0.0,        0.0,         1.0 ));
        

        float fur_length = (sin(angle) + 1.0) / 2.0;
        vec3 pos = inPosition;
        pos.x = (pos.x - 5.0 )* ( 1.0 -  inColor.r + 0.0 ) ;
        pos.z = pos.z + 1.0 * inColor.r;
        vec3 rotatedPoint =  rotation * vec3(pos * (fur_length + 0.5)) + vec3(instancePosition, 0.0);

        gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * vec4(rotatedPoint, 1.0);
        
        vColor = inColor;
    }   
    ";

    let frag_shader_string = "#version 410

    in vec4 vColor;

    uniform vec3 uTipColor;
    uniform vec3 uBaseColor;

    out vec4 Color;
    void main()
    {   
        float alpha = vColor.r;
        Color = vec4(mix(uBaseColor, uTipColor, vColor.r), alpha);
    }
    ";

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();

    // build vertex data ----
    let fur_width = 10.0;

    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![fur_width, 30.0, 0.0] );
    vertices.append( &mut vec![0.0,  30.0, 0.0,] );

    vertices.append( &mut vec![0.0,   0.0, 0.0] );
    vertices.append( &mut vec![fur_width, 30.0, 0.0] );
    vertices.append( &mut vec![fur_width, 0.0, 0.0] );
 

    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut colors : Vec<f32> = Vec::new();
    colors.append( &mut vec![0.0,   0.1, 0.1, 1.0]);
    colors.append( &mut vec![1.0, 0.9, 0.1, 1.0]);
    colors.append( &mut vec![1.0, 0.9, 0.1, 1.0]);

    colors.append( &mut vec![0.0, 0.1, 0.1, 1.0]);
    colors.append( &mut vec![1.0, 0.9, 0.1, 1.0]);
    colors.append( &mut vec![0.0, 0.1, 0.1,  1.0]);

    //create the instance position attribute buffer
    let mut instance_positions : Vec<f32> = Vec::new();
    let spacing = 10.;
    let mut rng = rand::thread_rng();

    let random_range = 1.0;

    let max_x = 550;
    let max_y = 190;

    for i in 0..max_x{
        for k in 0 ..max_y{

            let x = ((max_x as f32) - (i as f32)) + rng.gen_range(-random_range..random_range);
            let y = ((max_y as f32) - (k as f32)) + rng.gen_range(-random_range..random_range);
            instance_positions.append( &mut vec![x as f32 * spacing * 0.5, y as f32 * spacing ]);
        }
    }

    let number_of_instances = instance_positions.len() as i32 / 2;
    println!("number of instances: {}", number_of_instances);

    let instance_positions_attrib = glh::VertexAttrib{
        name : "instancePosition",
        size : 2,
        stride : 0, 
        data : instance_positions,
        per_instance : true,
    };

    pos_attrib.data = vertices;
    color_attrib.data = colors;

    let shader = glh::GlslProg::new(gl, vertex_shader_string, frag_shader_string);
    let attribs = vec![pos_attrib, color_attrib, instance_positions_attrib];
    let vao = glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap();

    FrameData{ 
        vao,
        shader,
        number_of_instances,
        time: 0.0,
    }
}

fn m_update(app : &mut app::App, _data : &mut FrameData, _event : &app::Event<()>)
{   

    let time = &mut _data.time;
    let gl = &app.gl;
    let shader = &_data.shader;
    let vao = &_data.vao;

    let mut mouse_pos : [f32; 2] = [0.0,0.0];

    let base_color : [f32; 3] = [0.2, 0.1, 0.1];
    let tip_color : [f32; 3] = [0.9, 0.0, 0.2];

    let framebuffer_scale = 2.0;
    let inv_frambe_buffer_scale = 1.0 / framebuffer_scale; 

    *time = *time + 1f32;
    glh::clear(gl, base_color[0], base_color[1], base_color[2], 1.0);

    mouse_pos[0] = mouse_pos[0] + ((app.input_state.mouse_pos.0  * 2.0)  - mouse_pos[0]) * 0.06;
    mouse_pos[1] = mouse_pos[1] + ((app.input_state.mouse_pos.1  * 2.0)  - mouse_pos[1]) * 0.06;


    unsafe{
        gl.enable( glow::DEPTH_TEST );
        gl.enable( glow::BLEND );
        gl.blend_func( glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA );
    }

    shader.bind(gl);

    shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(),
                            &glm::ortho(0.0,
                                app.settings.window_size.0 as f32 * inv_frambe_buffer_scale, // beacuse of mac dpi we need to scale it down
                                app.settings.window_size.1 as f32 * inv_frambe_buffer_scale,
                                0.0, -1.0,
                                1.0));

   shader.set_uniform_1f(gl, "uTime", *time);
   shader.set_uniform_2f(gl, "uMousePos", &mouse_pos);
   
   shader.set_uniform_3f(gl, "uBaseColor", &base_color);
   shader.set_uniform_3f(gl, "uTipColor", &tip_color);

    shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
    
    shader.set_uniform_mat4(gl,  glh::StockShader::uniform_name_model_matrix(), &model_view );

    vao.draw_instanced( gl, glow::TRIANGLES, _data.number_of_instances );

    shader.unbind(gl);
    
}

fn main() {
    app::AppBuilder::new(app::AppSettings{
        window_size : (1024, 768),
        window_title : "simple app",
    }, m_setup).run(m_update);
}