extern crate piralib;
use piralib::app;
use nalgebra_glm as glm;
use piralib::gl_helper as glh;

use glow::*;

struct FrameData { 
    program : glh::GlslProg,
    vao : glh::Vao,
}

fn m_setup(app : &mut app::App) -> FrameData {

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
    let shader = stock_shader.build(&app.gl);
    let attribs = vec![pos_attrib, color_attrib];
    let vao = glh::Vao::new_from_attrib_indexed(&app.gl, &attribs, &indices, &shader).unwrap();

    FrameData{ 
        program : shader,
        vao
    }
}

fn m_update(app : &mut app::App, _data : &mut FrameData, _event : &app::Event<()>)
{   
    glh::clear(&app.gl, 1.0, 0.0, 0.5, 1.0);
    glh::set_viewport(&app.gl, 0, 0, 1024, 768);


        let shader = &_data.program;
        let vao = &_data.vao;

        shader.bind(&app.gl);
        shader.set_uniform_mat4(&app.gl,  glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    1024 as f32,
                                    768 as f32,
                                    0.0, -0.0,
                                    1.0));

        shader.set_uniform_mat4(&app.gl, glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        // model_view = glm::translate(&model_view, &glm::vec3( 10.0, 0.0, 0.0 ));
        // model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4(&app.gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
      //  shader.set_uniform_4f(&app.gl, glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw(&app.gl, glow::TRIANGLES );

        shader.unbind(&app.gl);
}

fn main() {


    app::AppBuilder::new(m_setup).run(m_update)
    /*
    let mut app  = piralib::App::init_with_options( &piralib::app::Options{
        window_width: 1104,
        window_height: 736,
        samples : 4,
        title: "#️⃣".to_string()
    });

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

    app.run_fn(move |event, should_quit| {

        glh::clear(0.2, 0.1, 0.1, 1.0);

        shader.bind();
        shader.set_uniform_mat4( glh::StockShader::uniform_name_perspective_matrix(),
                                &glm::ortho(0.0,
                                    event.framebuffer_size.0 as f32 * 0.5,
                                    event.framebuffer_size.1 as f32 * 0.5,
                                    0.0, -1.0,
                                    1.0));

        shader.set_uniform_mat4( glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

        let mut model_view = glm::Mat4::identity();
        model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
        model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
        
        shader.set_uniform_mat4( glh::StockShader::uniform_name_model_matrix(), &model_view );
        shader.set_uniform_4f( glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

        vao.draw( gl::TRIANGLES );

        shader.unbind();
                                    
        if cfg!(test){
            if event.frame_number > 10 {
                let img = event.get_frame_image();
                let img = image::imageops::flip_vertical(&img);
                *should_quit = true;
                img.save("test_images/indexed.png").unwrap();   
            }
        }
    });
    */
}


#[test]
fn save_frame_test() {
    main();
}