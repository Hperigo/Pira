extern crate piralib;
use piralib::app;
use piralib::gl_helper as glh;

use glow;
use rand::*;

use piralib::egui;

//use nalgebra_glm as glm;
use glam;


// create a simple particle object
#[derive(Clone, Copy)]
pub struct Particle {
    position: glam::Vec3,
    speed: glam::Vec3,
    scale: f32,
    rotation: f32,
    lifetime: f32,
}

struct FrameData {
    particles: Vec<Particle>,
    vao: glh::Vao,
    shader: glh::GlslProg,
}

fn m_setup(app: &mut app::App) -> FrameData {
    //    let mut pos_attrib = glh::VertexAttrib::new_position_attr();
    //    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    //    let mut texture_attrib = glh::VertexAttrib::new_texture_attr();

    // build vertex data ----
    let mut vertices: Vec<f32> = Vec::new();
    vertices.append(&mut vec![-250.0, -250.0, 0.0]);
    vertices.append(&mut vec![250.0, -250.0, 0.0]);
    vertices.append(&mut vec![0.0, 350.0, 0.0]);
    //        pos_attrib.data = vertices;

    let mut colors: Vec<f32> = Vec::new();
    colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
    colors.append(&mut vec![0.9, 0.8, 0.9, 1.0]);
    colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
    //        color_attrib.data = colors;

    let mut texure_vertices: Vec<f32> = Vec::new();
    texure_vertices.append(&mut vec![0.5, 0.5, 0.0]);
    texure_vertices.append(&mut vec![0.5, 0.5, 1.0]);
    texure_vertices.append(&mut vec![1.0, 1.0, 1.0]);

    //texture_attrib.data = texure_vertices;

    let shader = glh::StockShader::new().color().build(&app.gl);

    let attribs = vec![
        glh::VertexAttrib::new_position_attr_with_data(&vertices),
        glh::VertexAttrib::new_color_attr_with_data(&colors),
    ];
    let vao = glh::Vao::new_from_attrib(&app.gl, &attribs, glow::TRIANGLES, &shader).unwrap();

    // Particles -------
    let particles: Vec<Particle> = Vec::new();

    FrameData {
        particles,
        vao,
        shader,
    }
}

fn m_update(app: &app::App, _data: &mut FrameData, _ui: &egui::Context) {
    let gl = &app.gl;
    let shader = &_data.shader;
    let vao = &_data.vao;

    glh::clear(gl, 0.2, 0.1, 0.1, 1.0);
    glh::set_viewport(
        gl,
        0,
        0,
        app.input_state.window_size.0,
        app.input_state.window_size.1,
    );

    shader.bind(gl);
    shader.set_orthographic_matrix(gl, &[app.input_state.window_size.0 as f32, app.input_state.window_size.1 as f32]);
 
    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_view_matrix(),
        &glam::Mat4::IDENTITY,
    );

    // update particles ----
    for p in &mut _data.particles {
        p.lifetime += 1.0;
        p.scale -= 0.001;

        p.position += p.speed;
        p.rotation += 0.01;

        let mat =  glam::Mat4::from( glam::Affine3A::from_scale_rotation_translation( glam::vec3(p.scale, p.scale, p.scale), glam::Quat::from_axis_angle(glam::vec3(0.0, 0.0, 1.0), p.rotation), p.position) );  //glm::Mat4::identity();

        shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_model_matrix(), &mat);

        let green: f32 = 0.5 * (p.scale * 10.0);
        let red: f32 = 0.8 * (p.scale * 10.0);
        let blue: f32 = 1.0 - (app.input_state.mouse_pos.0 / 400.0);
        shader.set_uniform_4f(
            gl,
            glh::StockShader::uniform_name_color(),
            &[red, green, blue, 1.0],
        );
        vao.draw(gl);
    }

    shader.unbind(gl);

    let xpos = app.input_state.mouse_pos.0;
    let ypos = app.input_state.mouse_pos.1;

    let mut rng = rand::thread_rng();
    let sx: f32 = rng.gen_range(-1.0..1.0);
    let sy: f32 = rng.gen_range(-5.0..-1.0);
    let r: f32 = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);

    _data.particles.push(Particle {
        position: glam::vec3(xpos as f32, ypos as f32, 0.0),
        speed: glam::vec3(sx, sy, 0.0),
        scale: 0.1,
        rotation: r,
        lifetime: 0.0,
    });

    _data.particles.retain(|&p| p.scale > 0.0);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (400, 650),
            window_title: "Hello",
        },
        m_setup,
    )
    .run(m_update)
}
