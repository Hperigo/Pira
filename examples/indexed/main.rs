extern crate piralib;
use piralib::app;
use piralib::gl_helper as glh;
use piralib::egui;
use glow::*;

struct FrameData {
    program: glh::GlslProg,
    vao: glh::Vao,
}

fn m_setup(app: &mut app::App) -> FrameData {
    let shader = glh::StockShader::new().color().build(&app.gl);

    let mut vertices: Vec<f32> = Vec::new();
    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![0.0, 1.0, 0.0]);
    vertices.append(&mut vec![1.0, 1.0, 0.0]);
    vertices.append(&mut vec![1.0, 0.0, 0.0]);

    let mut colors: Vec<f32> = Vec::new();
    colors.append(&mut vec![1.0, 0.0, 0.0, 1.0]);
    colors.append(&mut vec![0.0, 1.0, 0.0, 1.0]);
    colors.append(&mut vec![0.0, 0.0, 1.0, 1.0]);
    colors.append(&mut vec![0.0, 0.4, 0.4, 1.0]);

    let mut indices: Vec<u32> = Vec::new();
    indices.append(&mut vec![0, 1, 2]);
    indices.append(&mut vec![0, 2, 3]);

    // let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    // let mut color_attrib = glh::VertexAttrib::new_color_attr();
    // pos_attrib.data = vertices;
    // color_attrib.data = colors;

    let attribs = [
        glh::VertexAttrib::new_position_attr_with_data(&vertices),
        //color_attrib.to_vertex_attrib_slice(),
        glh::VertexAttrib::new_color_attr_with_data(&colors),
    ];

    // let vao =
    //     glh::Vao::new_from_attrib_indexed(&app.gl, &attribs, &indices, glow::TRIANGLES, &shader)
    //         .expect("unable to create main vao");

    let vao = glh::vao::Vao::new_from_attrib_indexed(
        &app.gl,
        &attribs[0..2],
        &indices,
        glow::TRIANGLES,
        &shader,
    )
    .expect("unable to create main vao");

    FrameData {
        program: shader,
        vao,
    }
}

fn m_update(app: &app::App, _data: &mut FrameData, _ui: &egui::Context) {
    glh::clear(&app.gl, 1.0, 0.0, 0.5, 1.0);
    unsafe {
        app.gl.disable(glow::CULL_FACE);
    };

    let shader = &_data.program;
    let vao = &_data.vao;

    shader.bind(&app.gl);

    shader.set_orthographic_matrix(&app.gl, &[1024.0, 768.0]);

    shader.set_uniform_mat4(
        &app.gl,
        glh::StockShader::uniform_name_view_matrix(),
        &glam::Mat4::IDENTITY,
    );
    let model_view = glam::Mat4::from( glam::Affine3A::from_scale_rotation_translation(glam::vec3(100.0, 100.0, 100.0), glam::Quat::IDENTITY, glam::vec3(1024.0/2.0, 768.0/2.0, 0.0)));

    shader.set_uniform_mat4(
        &app.gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view,
    );

    vao.draw(&app.gl);

    shader.unbind(&app.gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1024, 768),
            window_title: "Hello",
        },
        m_setup,
    )
    .run(m_update)
}

#[test]
fn save_frame_test() {
    main();
}
