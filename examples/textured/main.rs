extern crate piralib;
use image::EncodableLayout;
use image::ImageBuffer;
use image::Rgba;
use piralib::event::ElementState;
use piralib::gl_helper as glh;

use piralib::app;
use piralib::event;

use image;
use nalgebra_glm as glm;

struct FrameData {
    vao: glh::VaoSliced,
    shader: glh::GlslProg,
    texture: glh::Texture,

    current_index: usize,
    images: [ImageBuffer<Rgba<u8>, Vec<u8>>; 2],
}

fn m_setup(app: &mut app::App) -> FrameData {
    let gl = &app.gl;

    let img = image::open("assets/uv_image.png").unwrap().to_rgba8();
    println!("Image width: {:?} height: {:?}", img.width(), img.height());
    let texture =
        glh::Texture::new_from_image_rgbau8(gl, &img, glh::texture::TextureSettings::default());

    let mut img2 = image::RgbaImage::new(img.width(), img.height());

    for x in 0..img2.width() {
        for y in 0..img2.height() {
            let dtx = x as f32 / img2.width() as f32;
            let dty = y as f32 / img2.height() as f32;

            img2.put_pixel(
                x,
                y,
                image::Rgba([(250.0 * dtx) as u8, (250.0 * dty) as u8, 0, 255]),
            );
        }
    }

    // build vertex data ----
    let mut vertices: Vec<f32> = Vec::new();
    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![1024.0, 1024.0, 0.0]);
    vertices.append(&mut vec![0.0, 1024.0, 0.0]);

    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![1024.0, 1024.0, 0.0]);
    vertices.append(&mut vec![1024.0, 0.0, 0.0]);

    let mut colors: Vec<f32> = Vec::new();
    let mut texture_vertices: Vec<f32> = Vec::new();
    {
        let num_of_vertices = vertices.len();
        let mut i = 0;

        while i < num_of_vertices {
            colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
            texture_vertices.append(&mut vec![vertices[i] / 1024.0, vertices[i + 1] / 1024.0]); // normalize vertex coords
            i = i + 3;
        }
    }

    let shader = glh::StockShader::new().texture(false).build(gl);
    let attribs = vec![
        glh::VertexAttribSlice::new_position_attr_with_data(&vertices),
        glh::VertexAttribSlice::new_texture_attr_with_data(&texture_vertices),
    ];
    let vao = glh::VaoSliced::new_from_attrib(gl, &attribs, glow::TRIANGLES, &shader).unwrap();

    FrameData {
        vao,
        shader,
        texture,

        current_index: 0,
        images: [img, img2],
    }
}

fn m_event(_app: &mut app::App, _data: &mut FrameData, event: &event::WindowEvent) {
    if let event::WindowEvent::MouseInput { state, .. } = event {
        if matches!(state, event::ElementState::Pressed) {}
    }

    if let event::WindowEvent::KeyboardInput { input, .. } = event {
        if matches!(input.state, ElementState::Released) {
            _data.current_index = (_data.current_index + 1) % 2;
            _data
                .texture
                .update(&_app.gl, _data.images[_data.current_index].as_bytes());
        }
    }
}

fn m_update(app: &app::App, data: &mut FrameData, _ui: &egui::Context) {
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
            app.input_state.window_size.0 as f32 * frame_buffer_scale,
            app.input_state.window_size.0 as f32 * frame_buffer_scale,
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

    vao.draw(gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1024, 768),
            window_title: "simple app",
        },
        m_setup,
    )
    .event(m_event)
    .run(m_update);
}
