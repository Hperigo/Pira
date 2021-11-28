extern crate piralib;
use glow::*;
use nalgebra_glm as glm;
use piralib::app;
use piralib::gl_helper as glh;

struct FrameData {
    shader: glh::GlslProg,
    vao: glh::Vao,
}

fn m_setup(_app: &mut app::App) -> FrameData {
    let shader = glh::StockShader::new().color().build(&_app.gl); //glh::GlslProg::new(&_app.gl, vertex_source, frag_source);

    let geo = glh::Geometry::rect(0.0, 0.0, 100.0, 100.0);
    let vao = glh::Vao::new_from_attrib(&_app.gl, &geo.attribs, &shader).unwrap();

    FrameData { shader, vao }
}

fn m_update(app: &mut app::App, _data: &mut FrameData, _event: &app::Event<()>) {
    let gl = &app.gl;
    unsafe {
        app.gl.clear(glow::COLOR_BUFFER_BIT);
        app.gl.clear_color(1.0, 0.0, 0.4, 1.0);
    }

    _data.shader.bind(gl);
    _data.shader.set_orthographic_matrix(
        gl,
        [
            app.settings.window_size.0 as f32,
            app.settings.window_size.1 as f32,
        ],
    );

    _data.shader.set_view_matrix(gl, &glm::Mat4::identity());
    _data.shader.set_model_matrix(gl, &glm::Mat4::identity());
    _data.shader.set_color(gl, &[1.0, 0.0, 0.3, 1.0]);
    _data.vao.draw(gl, glow::TRIANGLES);

    _data.shader.unbind(gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (200, 200),
            window_title: "simple app",
        },
        m_setup,
    )
    .run(m_update);
}
