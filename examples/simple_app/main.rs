extern crate piralib;

use glam;

use piralib::app;
use piralib::gl_helper as glh;

use piralib::event;
use piralib::utils::geo::Circle;
use piralib::utils::geo::Geometry;

use piralib::egui;

struct FrameData {
    shader: glh::GlslProg,
    vao: glh::Vao,

    mouse_pos : glam::Vec3,
}

fn m_setup(app: &mut app::App) -> FrameData {
    let gl = &app.gl;
    let (vao, shader) = Circle::new(0.0, 0.0, 30.0).get_vao_and_shader(gl);
    FrameData { vao, shader, mouse_pos : glam::Vec3::ZERO }
}

fn m_event(_app: &mut app::App, data: &mut FrameData, event: &event::WindowEvent) {
 
    if let event::WindowEvent::CursorMoved{ position, .. } = event {
        data.mouse_pos = glam::vec3(position.x as f32, position.y as f32, 0.0);
    }

    /*
    if let event::WindowEvent::MouseInput { state, .. } = event {
        if matches!(state, event::ElementState::Pressed) {}
    }

    if let event::WindowEvent::KeyboardInput { input, .. } = event {
      match input.state {
          event::ElementState::Pressed => {
          },                                                                                                                                 
                                                                                                                                             
          event::ElementState::Released => {                                                                                                 
          }                                                                                                                                  
      }                                                                                                                                      
  }
  */
}

fn m_update(app: &app::App, data: &mut FrameData, _ui: &egui::Context) {
    let gl = &app.gl;
    let circle_shader = &data.shader;
    let circle_vao = &data.vao;

    glh::clear(gl, 1.0, 0.0, 0.4, 1.0);

    circle_shader.bind(gl);
    circle_shader.set_orthographic_matrix(
        gl,
        &[
            app.input_state.window_size.0 as f32,
            app.input_state.window_size.1 as f32,
        ],
    );

    circle_shader.set_view_matrix(gl, &glam::Mat4::IDENTITY);
    let model_view = glam::Mat4::from(glam::Affine3A::from_scale_rotation_translation(glam::vec3(1.0, 1.0, 1.0), glam::Quat::IDENTITY,  data.mouse_pos ));
    
    circle_shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view,
    );
    circle_shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &[1.0, 1.0, 1.0, 1.0],
    );

    circle_vao.draw(gl);

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
