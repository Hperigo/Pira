extern crate piralib;
use glow::HasContext;
use piralib::app;
use piralib::gl_helper as glh;
use piralib::egui;

use piralib::gl_helper::texture::TextureSettings;
use piralib::utils::geo;
use piralib::utils::geo::Geometry;

use piralib::gl_helper::Bindable;

use glam;

struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,

    circle_vao: glh::Vao,
    circle_shader: glh::GlslProg,

    quad_pos: glam::Vec3,
    circle_pos: glam::Vec3,
    circle_scale: glam::Vec3,

    fbo: glh::Fbo,
}

fn m_setup(app: &mut app::App) -> FrameData {
    let gl = &app.gl;
    let fbo = glh::Fbo::new(
        gl,
        glh::FboSettings {
            width: 1920 / 2,
            height: 1080 / 2,
            depth: true,
            initialize_default_texture: true,
        },
        TextureSettings::default(),
    );

    // create QUAD ====
    let (vao, shader) = geo::Rect::new(0.0, 0.0, fbo.get_width() as f32, fbo.get_height() as f32)
        .texture_coords()
        .get_vao_and_shader(gl);

    // create geomtry that is drawn inside the fbo ====
    let (circle_vao, circle_shader) = geo::Circle::new(0.0, 0.0, 100.0).get_vao_and_shader(gl);

    FrameData {
        vao,
        shader,

        circle_vao,
        circle_shader,

        quad_pos: glam::vec3(0.0, 0.0, 0.0),

        circle_pos: glam::vec3(0.0, 250.0, 0.0),
        circle_scale: glam::vec3(1.0, 1.0, 1.0),
        fbo,
    }
}

fn m_update(app: &app::App, _data: &mut FrameData, _ui: &egui::Context) {

    let FrameData {
        vao,
        circle_vao,
        circle_shader,
        shader,
        fbo,
        circle_pos,
        circle_scale,
        quad_pos,
    } = _data;

    let gl = &app.gl;

    #[cfg(not(target_arch = "wasm32"))]
    egui::Window::new("hello").show(_ui, |ui| {
        ui.columns(4, |ui_label| {
            ui_label[0].label("quad_pos");
            ui_label[1].add(egui::DragValue::new(&mut quad_pos.x));
            ui_label[2].add(egui::DragValue::new(&mut quad_pos.y));
            ui_label[3].add(egui::DragValue::new(&mut quad_pos.z));
        });

        ui.columns(4, |ui_label| {
            ui_label[0].label("circle_pos");
            ui_label[1].add(egui::DragValue::new(&mut circle_pos.x));
            ui_label[2].add(egui::DragValue::new(&mut circle_pos.y));
            ui_label[3].add(egui::DragValue::new(&mut circle_pos.z));
        });

        ui.columns(4, |ui_label| {
            ui_label[0].label("circle scale");
            ui_label[1].add(egui::DragValue::new(&mut circle_scale.x).speed(0.01));
            ui_label[2].add(egui::DragValue::new(&mut circle_scale.y).speed(0.01));
            ui_label[3].add(egui::DragValue::new(&mut circle_scale.z).speed(0.01));
        });
    });

    fbo.bind(gl);
    unsafe {
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
    }

    // glh::set_viewport(gl, 0,0, app.input_state.window_size.0 * frame_buffer_scale as i32, app.input_state.window_size.1 * frame_buffer_scale as i32);
    glh::set_viewport(
        gl,
        0,
        0,
        fbo.get_width() as i32,
        fbo.get_height() as i32,
    );
    glh::clear(gl, 0.0, 1.0, 0.0, 1.0);

    circle_shader.bind(gl);
    circle_shader.set_orthographic_matrix(gl, &[fbo.get_width() as f32, fbo.get_height() as f32]);

    circle_shader.set_view_matrix(gl, &glam::Mat4::IDENTITY);

    circle_shader.set_transform(gl, *circle_pos, glam::Quat::IDENTITY, *circle_scale);
    circle_shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &[1.0, 1.0, 1.0, 1.0],
    );
    circle_shader.set_color(gl, &[1.0, 0.0, 0.0, 1.0]);
    
    circle_vao.draw(gl);
    circle_shader.unbind(gl);
    fbo.unbind(gl);

    // DRAW FBO -------
    glh::set_viewport(
        gl,
        0,
        0,
        app.input_state.window_size.0,
        app.input_state.window_size.1,
    );
    glh::clear(gl, 0.3, 0.3, 0.3, 1.0);

    shader.bind(gl);

    shader.set_orthographic_matrix(
        gl,
        &[
            app.input_state.window_size.0 as f32,
            app.input_state.window_size.1 as f32,
        ],
    );

    shader.set_view_matrix(gl, &glam::Mat4::IDENTITY);
    let model_view = glam::Affine3A::from_translation(*quad_pos); // glam::Mat4::from( glam::Affine3A::from_scale_rotation_translation( glam::vec3(1.5, 1.5, 1.5), glam::Quat::IDENTITY, *quad_pos) );

    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view.into(),
    );
    shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &[1.0, 1.0, 1.0, 1.0],
    );

    fbo.bind_texture(gl);
    vao.draw(gl);
    fbo.unbind_texture(gl);

    shader.unbind(gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1920 / 2, 1080 / 2),
            window_title: "FBO app",
        },
        m_setup,
    )
    .run(m_update);
}
