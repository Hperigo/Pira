extern crate piralib;
use piralib::gl_helper as glh;

use piralib::app;

use image;
use nalgebra_glm as glm;

struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,
    texture: glh::Texture,
}

fn m_setup(app: &mut app::App) -> FrameData {
    let gl = &app.gl;

    let img = image::open("assets/uv_image.png").unwrap().to_rgba8();
    println!("Image width: {:?} height: {:?}", img.width(), img.height());
    let texture = glh::Texture::new_from_image_rgbau8(gl, &img);

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

    // build vertex data ----
    let mut vertices: Vec<f32> = Vec::new();
    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![1024.0, 1024.0, 0.0]);
    vertices.append(&mut vec![0.0, 1024.0, 0.0]);

    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![1024.0, 1024.0, 0.0]);
    vertices.append(&mut vec![1024.0, 0.0, 0.0]);

    let mut colors: Vec<f32> = Vec::new();
    let mut texure_vertices: Vec<f32> = Vec::new();
    {
        let num_of_vertices = vertices.len();
        let mut i = 0;

        while i < num_of_vertices {
            colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
            texure_vertices.append(&mut vec![vertices[i] / 1024.0, vertices[i + 1] / 1024.0]); // normalize vertex coords
            i = i + 3;
        }
    }

    pos_attrib.data = vertices;
    color_attrib.data = colors;
    texture_attrib.data = texure_vertices;
    let shader = glh::StockShader::new().texture(false).build(gl);
    let attribs = vec![pos_attrib, texture_attrib];
    let vao = glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap();

    FrameData {
        vao,
        shader,
        texture,
    }
}

fn m_update(app: &mut app::App, data: &mut FrameData, _ui : &egui::CtxRef) {
    let gl = &app.gl;
    let shader = &data.shader;
    let vao = &data.vao;
    let texture = &data.texture;

    glh::clear(gl, 0.2, 0.1, 0.1, 1.0);

    let _s_shader = glh::ScopedBind::new(gl, shader);
    let _s_tex = glh::ScopedBind::new(gl, texture);

    let frame_buffer_scale = 1.0;
    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_perspective_matrix(),
        &glm::ortho(
            0.0,
            app.settings.window_size.0 as f32 * frame_buffer_scale,
            app.settings.window_size.0 as f32 * frame_buffer_scale,
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
    model_view = glm::translate(&model_view, &glm::vec3(0.0, 0.0, 0.0));
    model_view = glm::scale(&model_view, &glm::vec3(0.5, 0.5, 0.5));

    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_model_matrix(),
        &model_view,
    );
    shader.set_uniform_4f(
        gl,
        glh::StockShader::uniform_name_color(),
        &[1.0, 1.0, 1.0, 1.0],
    );

    vao.draw(gl, glow::TRIANGLES);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1024, 768),
            window_title: "simple app",
        },
        m_setup,
    )
    .run(m_update);
}
