extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use nalgebra_glm as glm;
use std::ffi::CString;

use rand::Rng;
use imgui_glfw_rs::imgui::*;

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        samples : 2,
        title: "#️⃣".to_string()
    });

   let vertex_shader_string = "#version 330

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
        

        vec3 pos = inPosition;
        pos.x = (pos.x - 5.0 )* ( 1.0 -  inColor.r + 0.0 ) ;
        pos.z = pos.z + 1.0 * inColor.r;
        vec3 rotatedPoint =  rotation * vec3(pos) + vec3(instancePosition, 0.0);

        gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * vec4(rotatedPoint, 1.0);
        
        vColor = inColor;
    }   
    ";

    let frag_shader_string = "#version 330
    uniform vec4 uColor;

    in vec4 vColor;

    uniform vec3 uTipColor;
    uniform vec3 uBaseColor;

    out vec4 Color;
    void main()
    {   
        float alpha = 1.0; // - vColor.r;
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

            let x = ((max_x as f32) - (i as f32)) + rng.gen_range(-random_range, random_range);
            let y = ((max_y as f32) - (k as f32)) + rng.gen_range(-random_range, random_range);
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

    let shader = glh::GlslProg::new(&CString::new(vertex_shader_string).unwrap(), &CString::new(frag_shader_string).unwrap());
    let attribs = vec![pos_attrib, color_attrib, instance_positions_attrib];
    let vao = glh::Vao::new_from_attrib(&attribs, &shader).unwrap();

    let mut frame_incc = 1.0;
    let mut time = 0.0;
    let mut mouse_pos : [f32; 2] = [0.0,0.0];

    let mut base_color : [f32; 3] = [0.2, 0.1, 0.1];
    let mut tip_color : [f32; 3] = [0.9, 0.0, 0.2];

    let framebuffer_scale = app.get_framebuffer_scale();
    let inv_frambe_buffer_scale = 1.0 / framebuffer_scale; // used for the glViewport

    while app.run(){

        time = time + frame_incc;
        glh::clear(base_color[0], base_color[1], base_color[2], 1.0);

        mouse_pos[0] = mouse_pos[0] + ((app.mouse_pos.x  * 2.0)  - mouse_pos[0]) * 0.06;
        mouse_pos[1] = mouse_pos[1] + ((app.mouse_pos.y  * 2.0)  - mouse_pos[1]) * 0.06;

    
        unsafe{
            gl::Enable( gl::BLEND );
            gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA );
            gl::Viewport(0,0, app.get_framebuffer_size().0, app.get_framebuffer_size().1);
        }

        shader.bind();

        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32 * inv_frambe_buffer_scale, // beacuse of mac dpi we need to scale it down
                                    app.get_framebuffer_size().1 as f32 * inv_frambe_buffer_scale,
                                    0.0, -1.0,
                                    1.0));

       shader.set_uniform_1f("uTime", time);
       shader.set_uniform_2f("uMousePos", &mouse_pos);
       
       shader.set_uniform_3f("uBaseColor", &base_color);
       shader.set_uniform_3f("uTipColor", &tip_color);

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw_instanced( gl::TRIANGLES, number_of_instances );

        shader.unbind();
        
        let mut save_frame = false;
        app.do_ui( |ui| {

            ui.text(im_str!("Settings:"));
            ui.drag_float3(im_str!("background color"), &mut base_color).speed(0.01).min(0.0).max(1.0).build();
            ui.drag_float3(im_str!("tip color"), &mut tip_color).speed(0.01).min(0.0).max(1.0).build();
            ui.drag_float(im_str!("speed"), &mut frame_incc).speed(0.01).min(0.001).max(5.0).build();


            if ui.button(im_str!("Save frame"), [0.0, 0.0]){
                save_frame = true;
            }
        } );


        if cfg!(test){
            if app.get_frame_number() > 10 {
                let img = app.get_frame_image();
                let img = image::imageops::flip_vertical(&img);
                img.save("test_images/instances.png").unwrap();
                return;
            }
        }
    }
}

#[test]
fn run_app() {
    main();


}