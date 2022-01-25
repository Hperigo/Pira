extern crate piralib;

use piralib::glow;
use piralib::gl_helper as glh;
use piralib::nalgebra_glm as glm;
use piralib::app;

use piralib::event;

struct FrameData {
    shader : glh::GlslProg,
    vao : glh::Vao,
}

fn m_setup(app: &mut app::App) -> FrameData {

    let gl = &app.gl;

    let geo = glh::geo::Geometry::circle(0.0, 0.0, 10.0);
    let shader = glh::StockShader::new().build(gl);
    let vao = glh::Vao::new_from_attrib(gl, &geo.attribs, &shader).unwrap();

    FrameData {
        vao,
        shader
    }
}

fn m_event( _app : &mut app::App, _data : &mut FrameData, event : &event::WindowEvent ){

    if let event::WindowEvent::MouseInput { state, ..} = event {
        if matches!( state, event::ElementState::Pressed ){
        }
    }

    // if let event::WindowEvent::CursorMoved{ position, .. } = event {
    // }

    // if let event::WindowEvent::KeyboardInput { .. } = event {
    // }
}

fn m_update(
    app: &mut app::App,
    data: &mut FrameData,
    _ui: &egui::CtxRef,
) {
    let gl = &app.gl;
    let circle_shader = &data.shader;
    let circle_vao = &data.vao;

    glh::clear(gl,1.0, 0.0, 0.4, 1.0);

    circle_shader.bind(gl);
    circle_shader.set_orthographic_matrix(
        gl,
        [
            app.input_state.window_size.0 as f32 * 2.0,
            app.input_state.window_size.1 as f32 * 2.0,
        ],
    );

    circle_shader.set_view_matrix(gl, &glm::Mat4::identity());

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3(200.0, 200.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(10.0, 10.0, 10.0));
    
    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    circle_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0] );

    circle_vao.draw(gl, glow::TRIANGLE_FAN);
    
    circle_shader.unbind(gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (200, 200),
            window_title: "simple app",
        },
        m_setup,
    )
    .event(m_event)
    .run(m_update);
}
