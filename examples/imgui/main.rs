extern crate piralib;
use piralib::gl_helper as glh;
use piralib::gl as gl;
use nalgebra_glm as glm;

use imgui_glfw_rs::imgui::*;

fn main() {

    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        samples : 4,
        title: "#️⃣".to_string()
    });


    let mut r : f32 = 0.1;
    let mut g : f32 = 0.2;
    let mut b : f32 = 0.3;

    let mut translation = [0.0, 0.0, 0.0];
    let mut scale = [0.5, 0.5, 0.5];
    let mut rotation = 0.0;

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();

    // build vertex data ----
    let mut vertices : Vec<f32> = Vec::new();
    vertices.append( &mut vec![0.0, 0.0, 0.0] );
    vertices.append( &mut vec![0.0, 736.0, 0.0,] );
    vertices.append( &mut vec![1104.0, 736.0, 0.0] );
    vertices.append( &mut vec![1104.0, 0.0, 0.0] );

    let mut colors : Vec<f32> = Vec::new();
    colors.append( &mut vec![1.0, 0.0, 0.0, 1.0] );
    colors.append( &mut vec![0.0, 1.0, 0.0, 1.0] );
    colors.append( &mut vec![0.0, 0.0, 1.0, 1.0] );
    colors.append( &mut vec![0.0, 0.4, 0.4, 1.0] );

    let mut indices : Vec<u32> = Vec::new(); 
    indices.append( &mut vec![0,2,3] );
    indices.append( &mut vec![0,1,2] );

    pos_attrib.data = vertices;
    color_attrib.data = colors;

    let stock_shader = glh::StockShader::new().color();
    let shader = stock_shader.build();
    let attribs = vec![pos_attrib, color_attrib];
    let vao = glh::Vao::new_from_attrib_indexed(&attribs, &indices, &shader).unwrap();

    unsafe{ 
       gl::Enable(gl::BLEND);
       gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    while app.run() {

        glh::clear(r, g, b, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    app.get_framebuffer_size().0 as f32,
                                    app.get_framebuffer_size().1 as f32,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( translation[0], translation[1], translation[2]));
        model_view = glm::rotate( &model_view, rotation, &glm::vec3(0.0,0.0,1.0) );
        model_view = glm::scale(&model_view, &glm::vec3(scale[0], scale[1], scale[2]));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw( gl::TRIANGLES );
        app.do_ui( |ui| {
            
            ui.drag_float(im_str!("R"), &mut r).speed(0.001).build();
            ui.drag_float(im_str!("G"), &mut g).speed(0.001).build();
            ui.drag_float(im_str!("B"), &mut b).speed(0.001).build();

            ui.drag_float3( im_str!("Translation"), &mut translation ).build();
            ui.drag_float3( im_str!("Scale"), &mut scale ).speed(0.01).build();
            ui.drag_float( im_str!("Rotation"), &mut rotation ).speed(0.01).build();
        } );
           
        shader.unbind();
    }
}
