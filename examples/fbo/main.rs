extern crate piralib;
use glow::*;
use nalgebra_glm as glm;
use piralib::app;
use piralib::gl_helper as glh;
use piralib::gl_helper::Bindable;
use piralib::gl_helper::Geometry;

struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,

    circle_vao : glh::Vao,
    circle_shader : glh::GlslProg,

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
            width: 500,
            height: 500,
            depth: true,
        },
    );

    // create QUAD ====
    let (vao, shader) = {
        
        let geometry = Geometry::rect(0.0, 0.0, fbo.get_width() as f32, fbo.get_height() as f32);
        let stock_shader = glh::StockShader::new().texture(true).color();
        let shader = stock_shader.build(gl);

        (
            glh::Vao::new_from_attrib(gl, &geometry.attribs, &shader).unwrap(),
            shader,
        )
    };

    // create geomtry that is drawn inside the fbo ====
    let (circle_vao, circle_shader) = {

        let circle = glh::Geometry::circle(0.0, 0.0, 60.0);        
        let stock_shader = glh::StockShader::new();
        let shader = stock_shader.build(gl);

        (glh::Vao::new_from_attrib(gl, &circle.attribs, &shader).unwrap(), shader)
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
    _event: &app::Event<()>,
    _ui: &egui::CtxRef,
) {
    let frame_buffer_scale = 1.0;

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


    glh::set_viewport(
        gl,
        0,
        0,
        app.settings.window_size.0 * 2,
        app.settings.window_size.1 * 2,
    );
    
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
            ui_label[1].add(egui::DragValue::new(&mut circle_scale.x).speed(0.1));
            ui_label[2].add(egui::DragValue::new(&mut circle_scale.y).speed(0.1));
            ui_label[3].add(egui::DragValue::new(&mut circle_scale.z).speed(0.1));
        });
    });

   let frame_buffer_scale = 1.0;

   unsafe{
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
   }


    fbo.bind(gl);
        
    glh::clear(gl, 0.7, 0.0, 0.1, 1.0);
    glh::set_viewport(gl, 0,0, fbo.get_width(), fbo.get_height() );

    circle_shader.bind(gl);
    circle_shader.set_orthographic_matrix(gl, 
                                [fbo.get_width() as f32 * frame_buffer_scale,
                                      fbo.get_height() as f32 * frame_buffer_scale]);

    circle_shader.set_view_matrix(gl, &glm::Mat4::identity());

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &circle_pos);
    model_view = glm::scale(&model_view, &glm::vec3(1.0,1.0, 1.0));
    
    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    circle_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0]);

    circle_vao.draw(gl, glow::TRIANGLE_FAN);
    circle_shader.unbind(gl);
    fbo.unbind(gl);


    // DRAW FBO -------
    glh::set_viewport(gl, 0,0, app.settings.window_size.0 * 2 as i32, app.settings.window_size.1 * 2);
    glh::clear(gl, 0.2, 0.1, 0.3, 1.0);

    shader.bind(gl);

    shader.set_orthographic_matrix(gl, 
                                [app.settings.window_size.0 as f32 * frame_buffer_scale,
                                      app.settings.window_size.1 as f32 * frame_buffer_scale]);

    shader.set_view_matrix(gl, &glm::Mat4::identity());

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &quad_pos);
    model_view = glm::scale(&model_view, &glm::vec3(0.5,0.5, 0.5));
    
    shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0]);

    fbo.bind_texture(gl);
    vao.draw( gl, glow::TRIANGLES );
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
