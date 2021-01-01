extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use nalgebra_glm as glm;

use image;

use imgui_glfw_rs::imgui::*;

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        title: "#️⃣".to_string(),
        samples: 2,
    });
    let fbo = glh::Fbo::new(glh::FboSettings{width : 500, height : 500, depth : true});
 
    // create QUAD ====
    let (vao, shader) = {
        let mut pos_attrib = glh::VertexAttrib::new_position_attr();
        let mut color_attrib = glh::VertexAttrib::new_color_attr();
        let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

        // build vertex data ----
        let mut vertices : Vec<f32> = Vec::new();
        vertices.append( &mut vec![0.0,   0.0, 0.0] );
        vertices.append( &mut vec![fbo.get_width() as f32, fbo.get_height() as f32, 0.0] );
        vertices.append( &mut vec![0.0,                    fbo.get_height() as f32, 0.0,] );

        vertices.append( &mut vec![0.0,   0.0, 0.0] );
        vertices.append( &mut vec![fbo.get_width() as f32, fbo.get_height() as f32, 0.0] );
        vertices.append( &mut vec![fbo.get_width() as f32, 0.0, 0.0] );

        let mut colors : Vec<f32> = Vec::new();
        let mut texure_vertices : Vec<f32> = Vec::new();
        {   
            let num_of_vertices = vertices.len();
            let mut i = 0;

            while i < num_of_vertices {
                colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
                texure_vertices.append( &mut vec![ vertices[i] / fbo.get_width() as f32, vertices[i+1]/ fbo.get_height() as f32 ] ); // normalize vertex coords
                i = i + 3;
            }
        }

        pos_attrib.data = vertices;
        color_attrib.data = colors;
        texture_attrib.data = texure_vertices;
        let stock_shader = glh::StockShader::new().texture(true);
        let shader = stock_shader.build();
        let attribs = vec![pos_attrib, texture_attrib];

        (glh::Vao::new_from_attrib(&attribs, &shader).unwrap(), shader)
    };

        // create geomtry that is drawn inside the fbo ====
    let (quad_vao, circle_shader) = {

        let mut pos_attrib = glh::VertexAttrib::new_position_attr();
        let mut color_attrib = glh::VertexAttrib::new_color_attr();
        let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

        // build vertex data ----
        let mut vertices : Vec<f32> = Vec::new();
        vertices.append( &mut vec![0.0,   0.0, 0.0] );

        for i in 0..33 {
            let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * 60.0;
            let y = angle.sin() * 60.0;

            vertices.append( &mut vec![x, y, 0.0] );    
        }
        
        pos_attrib.data = vertices;
        let stock_shader = glh::StockShader::new();
        let shader = stock_shader.build();
        let attribs = vec![pos_attrib];

        (glh::Vao::new_from_attrib(&attribs, &shader).unwrap(), shader)
    };

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
    }


    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    let frame_buffer_scale = 1.0;

    app.run_fn( move |event, should_quit| {

        fbo.bind();
        
        glh::clear(0.2, 0.1, 0.0, 1.0);
        glh::set_viewport(0,0, fbo.get_width(), fbo.get_height() );

        circle_shader.bind();
        //texture.bind();
        circle_shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    fbo.get_width() as f32 * frame_buffer_scale,
                                    fbo.get_height() as f32 * frame_buffer_scale,
                                    0.0, -1.0,
                                    1.0));

        circle_shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( x_pos, y_pos, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(1.0,1.0, 1.0));
        
        circle_shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        circle_shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        //qavao.draw( gl::TRIANGLES );
        quad_vao.draw(gl::TRIANGLE_FAN);
        //texture.unbind();
        circle_shader.unbind();
        fbo.unbind();


        // DRAW FBO -------
        unsafe{
                gl::Viewport(0,0, event.framebuffer_size.0, event.framebuffer_size.1);
        }

        glh::clear(0.2, 0.1, 0.3, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    event.framebuffer_size.0 as f32 * frame_buffer_scale,
                                    event.framebuffer_size.1 as f32 * frame_buffer_scale,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(1.0,1.0, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        fbo.bind_texture();
        vao.draw( gl::TRIANGLES );
        fbo.unbind_texture();

        shader.unbind();
        // END DRAW FBO -----

        event.ui.drag_float(im_str!("xpos"), &mut x_pos).speed(1.0).build();
        event.ui.drag_float(im_str!("ypos"), &mut y_pos).speed(1.0).build();


        if cfg!(test){
            if event.frame_number > 10 {
               
                let img = event.get_frame_image();
                let img = image::imageops::flip_vertical(&img);
                img.save("test_images/fbo.png").unwrap();
    
                *should_quit = true;
            }
        }
    });
}

#[test]
fn fbo_app() {
    main();
}