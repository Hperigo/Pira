extern crate piralib;
use piralib::app;
use piralib::gl_helper as glh;
use nalgebra_glm as glm;
use glow::*;
use piralib::gl_helper::Bindable;

struct FrameData { 
    vao : glh::Vao,
    shader : glh::GlslProg,

    quad_vao : glh::Vao,
    circle_shader : glh::GlslProg,


    fbo : glh::Fbo,
}

fn m_setup( app : &mut app::App) -> FrameData {

    let gl = &app.gl;
    let fbo = glh::Fbo::new(gl, glh::FboSettings{width : 500, height : 500, depth : true});

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
        let shader = stock_shader.build(gl);
        let attribs = vec![pos_attrib, texture_attrib];

        (glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap(), shader)
    };

          // create geomtry that is drawn inside the fbo ====
    let (quad_vao, circle_shader) = {

        let mut pos_attrib = glh::VertexAttrib::new_position_attr();

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
        let shader = stock_shader.build(gl);
        let attribs = vec![pos_attrib];

        (glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap(), shader)
    };
    

    FrameData{
        vao,
        shader,

        quad_vao,
        circle_shader,

        fbo,
     }
}

fn m_update(app : &mut app::App, _data : &mut FrameData, _event : &app::Event<()>, _ui : &egui::CtxRef)
{   
    let frame_buffer_scale = 1.0;

    
    let gl = &app.gl;

    let vao = &_data.vao;
    let quad_vao = &_data.quad_vao;
    let shader  = &_data.shader;
    let circle_shader = &_data.circle_shader;
    let fbo = &_data.fbo;
    unsafe {
        gl.clear( glow::COLOR_BUFFER_BIT );
        gl.clear_color(1.0, 0.0, 0.4, 1.0);
    }


    fbo.bind(gl);
        
    glh::clear(gl, 0.2, 0.1, 0.0, 1.0);
    glh::set_viewport(gl, 0,0, fbo.get_width(), fbo.get_height() );

    circle_shader.bind(gl);
    circle_shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(),
                            &glm::ortho(0.0,
                                fbo.get_width() as f32 * frame_buffer_scale,
                                fbo.get_height() as f32 * frame_buffer_scale,
                                0.0, -1.0,
                                1.0));

    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3( 10.0, 10.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(1.0,1.0, 1.0));
    
    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    circle_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

    quad_vao.draw(gl, glow::TRIANGLE_FAN);
    circle_shader.unbind(gl);
    fbo.unbind(gl);


    // DRAW FBO -------
    glh::set_viewport(gl, 0,0, app.settings.window_size.0, app.settings.window_size.1);
    glh::clear(gl, 0.2, 0.1, 0.3, 1.0);

    shader.bind(gl);
    shader.set_uniform_mat4(gl,  glh::StockShader::uniform_name_perspective_matrix(),
                            &glm::ortho(0.0,
                                app.settings.window_size.0 as f32 * frame_buffer_scale,
                               app.settings.window_size.0 as f32 * frame_buffer_scale,
                                0.0, -1.0,
                                1.0));

    shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_view_matrix(), &glm::Mat4::identity() );

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3( 0.0, 0.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(1.0,1.0, 0.5));
    
    shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &glm::vec4(1.0, 1.0, 1.0, 1.0));

    fbo.bind_texture(gl);
    vao.draw( gl, glow::TRIANGLES );
    fbo.unbind_texture(gl);

    shader.unbind(gl);

}

fn main() {
    app::AppBuilder::new(app::AppSettings{
        window_size : (1920 / 2,1080 / 2),
        window_title : "FBO app",
    }, m_setup).run(m_update);
}