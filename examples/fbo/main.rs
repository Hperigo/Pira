extern crate piralib;
use glow::*;
use nalgebra_glm as glm;
use piralib::app;
use piralib::egui::CtxRef;
use piralib::gl_helper as glh;
use piralib::gl_helper::Bindable;

use piralib::utils::geo::Geometry;
use piralib::utils::geo;
struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,

    circle_vao: glh::Vao,
    circle_shader: glh::GlslProg,

    quad_pos: glm::Vec3,
    circle_pos: glm::Vec3,
    circle_scale: glm::Vec3,

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
        },
    );

    // create QUAD ====
    let (vao, shader) = {
        
        let geometry = geo::Rect::new(0.0, 0.0, fbo.get_width() as f32, fbo.get_height() as f32).texture_coords().get_vertex_attribs();
        let stock_shader = glh::StockShader::new().texture(true);

        let shader =  stock_shader.build(gl);

        (
            glh::Vao::new_from_attrib(gl, &geometry, glow::TRIANGLES, &shader).unwrap(),
            shader,
        )
    };

    // create geomtry that is drawn inside the fbo ====
    let (circle_vao, circle_shader) = {
        let mut circle = geo::Circle::new(0.0, 0.0, 100.0); // Geometry::rect(0.0, 0.0, 200.0, 200.0,false); //geometry::circle(0.0, 0.0, 500.0);
        let stock_shader = glh::StockShader::new();

        let shader =  stock_shader.build(gl);

        (
            glh::Vao::new_from_attrib(gl, &circle.get_vertex_attribs(), glow::TRIANGLE_FAN, &shader).unwrap(),
            shader,
        )
    };

    FrameData {
        vao,
        shader,

        circle_vao,
        circle_shader,

        quad_pos: glm::vec3(0.0, 0.0, 0.0),

        circle_pos: glm::vec3(0.0, 0.0, 0.0),
        circle_scale: glm::vec3(1.0, 1.0, 1.0),
        fbo,
    }
}

fn m_update(
    app: &mut app::App,
    _data: &mut FrameData,
    _ui: &CtxRef,
) {
    let frame_buffer_scale = app.get_dpi_factor();

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
    
    #[cfg(not(target_arch="wasm32"))]
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
     unsafe{
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
    }
       
    // glh::set_viewport(gl, 0,0, app.input_state.window_size.0 * frame_buffer_scale as i32, app.input_state.window_size.1 * frame_buffer_scale as i32);
    glh::set_viewport(gl, 0,0, fbo.get_width() * frame_buffer_scale as i32, fbo.get_height() * frame_buffer_scale as i32);
    glh::clear(gl, 0.0, 1.0, 0.0, 1.0);

    circle_shader.bind(gl);
    circle_shader.set_orthographic_matrix(
        gl,
        [
            fbo.get_width() as f32,
            fbo.get_height() as f32,
        ],
    );

    circle_shader.set_view_matrix(gl, &glm::Mat4::identity());

    circle_shader.set_transform(gl, &circle_pos, &glm::vec3(0.0, 0.0, 0.0), &circle_scale);
    circle_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0]);
    circle_shader.set_color(gl, &[1.0, 0.0, 0.0, 1.0]);

    circle_vao.draw(gl);
    //vao.draw(gl, glow::TRIANGLES);
    circle_shader.unbind(gl);
    fbo.unbind(gl);

    // DRAW FBO -------
    glh::set_viewport(gl, 0,0, app.input_state.window_size.0 * frame_buffer_scale as i32, app.input_state.window_size.1 * frame_buffer_scale as i32);
    glh::clear(gl, 0.0, 0.0, 1.0, 1.0);
 


    shader.bind(gl);


    shader.set_orthographic_matrix(
        gl,
        [
            app.input_state.window_size.0 as f32 * frame_buffer_scale,
            app.input_state.window_size.1 as f32 * frame_buffer_scale,
        ],
    );

    shader.set_view_matrix(gl, &glm::Mat4::identity());

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &quad_pos);
    model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
    
    shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0]);


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
