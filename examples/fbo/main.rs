extern crate piralib;
use glow::*;
use nalgebra_glm as glm;
use piralib::app;
use piralib::gl_helper as glh;
use piralib::gl_helper::Bindable;

struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,

    quad_vao: glh::Vao,
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
            width: 500,
            height: 500,
            depth: true,
        },
    );

    // create QUAD ====
    let (vao, shader) = {
        let mut pos_attrib = glh::VertexAttrib::new_position_attr();
        let mut color_attrib = glh::VertexAttrib::new_color_attr();
        let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

        // build vertex data ----
        let mut vertices: Vec<f32> = Vec::new();
        vertices.append(&mut vec![0.0, 0.0, 0.0]);
        vertices.append(&mut vec![
            fbo.get_width() as f32,
            fbo.get_height() as f32,
            0.0,
        ]);
        vertices.append(&mut vec![0.0, fbo.get_height() as f32, 0.0]);

        vertices.append(&mut vec![0.0, 0.0, 0.0]);
        vertices.append(&mut vec![
            fbo.get_width() as f32,
            fbo.get_height() as f32,
            0.0,
        ]);
        vertices.append(&mut vec![fbo.get_width() as f32, 0.0, 0.0]);

        let mut colors: Vec<f32> = Vec::new();
        let mut texure_vertices: Vec<f32> = Vec::new();
        {
            let num_of_vertices = vertices.len();
            let mut i = 0;

            while i < num_of_vertices {
                colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
                texure_vertices.append(&mut vec![
                    vertices[i] / fbo.get_width() as f32,
                    vertices[i + 1] / fbo.get_height() as f32,
                ]); // normalize vertex coords
                i = i + 3;
            }
        }

        pos_attrib.data = vertices;
        color_attrib.data = colors;
        texture_attrib.data = texure_vertices;
        let stock_shader = glh::StockShader::new().texture(true);
        let shader = stock_shader.build(gl);
        let attribs = vec![pos_attrib, texture_attrib];

        (
            glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap(),
            shader,
        )
    };

    // create geomtry that is drawn inside the fbo ====
    let (quad_vao, circle_shader) = {
        let mut pos_attrib = glh::VertexAttrib::new_position_attr();

        // build vertex data ----
        let mut vertices: Vec<f32> = Vec::new();
        vertices.append(&mut vec![0.0, 0.0, 0.0]);

        for i in 0..33 {
            let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * 60.0;
            let y = angle.sin() * 60.0;

            vertices.append(&mut vec![x, y, 0.0]);
        }

        pos_attrib.data = vertices;
        let stock_shader = glh::StockShader::new();
        let shader = stock_shader.build(gl);
        let attribs = vec![pos_attrib];

        (
            glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap(),
            shader,
        )
    };

    FrameData {
        vao,
        shader,

        quad_vao,
        circle_shader,

        fbo,

        quad_pos: glm::vec3(0.0, 0.0, 0.0),
        circle_pos: glm::vec3(0.0, 0.0, 0.0),
        circle_scale: glm::vec3(1.0, 1.0, 1.0),
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
        quad_vao,
        circle_shader,
        shader,
        fbo,
        circle_pos,
        circle_scale,
        quad_pos,
    } = _data;

    let gl = &app.gl;

    unsafe {
        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.clear_color(8.0, 0.0, 0.4, 1.0);
    }

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

    fbo.bind(gl);

    glh::clear(gl, 0.7, 0.1, 0.2, 1.0);
    glh::set_viewport(gl, 0, 0, fbo.get_width(), fbo.get_height());

    circle_shader.bind(gl);
    circle_shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_perspective_matrix(),
        &glm::ortho(
            0.0,
            fbo.get_width() as f32 * frame_buffer_scale,
            fbo.get_height() as f32 * frame_buffer_scale,
            0.0,
            -1.0,
            1.0,
        ),
    );

    circle_shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_view_matrix(),
        &glm::Mat4::identity(),
    );

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &circle_pos);
    model_view = glm::scale(&model_view, &circle_scale);

    circle_shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view,
    );
    circle_shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &glm::vec4(1.0, 1.0, 1.0, 1.0),
    );

    quad_vao.draw(gl, glow::TRIANGLE_FAN);
    circle_shader.unbind(gl);
    fbo.unbind(gl);

    // DRAW FBO -------
    glh::set_viewport(
        gl,
        0,
        0,
        app.settings.window_size.0 * 2,
        app.settings.window_size.1 * 2,
    );
    glh::clear(gl, 0.2, 0.1, 0.3, 1.0);

    shader.bind(gl);
    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_perspective_matrix(),
        &glm::ortho(
            0.0,
            app.settings.window_size.0 as f32 * frame_buffer_scale,
            app.settings.window_size.1 as f32 * frame_buffer_scale,
            0.0,
            -1.0,
            1.0,
        ),
    );

    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_view_matrix(),
        &glm::Mat4::identity(),
    );

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &_data.quad_pos);
    model_view = glm::scale(&model_view, &glm::vec3(1.0, 1.0, 1.0));

    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view,
    );
    shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &glm::vec4(1.0, 1.0, 1.0, 1.0),
    );

    fbo.bind_texture(gl);
    vao.draw(gl, glow::TRIANGLES);
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
