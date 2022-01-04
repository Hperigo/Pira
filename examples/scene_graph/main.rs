extern crate piralib;
use nalgebra_glm as glm;
use piralib::gl_helper as glh;

use piralib::app::*;
use indextree::{Arena, NodeId};
use piralib::gl_helper::GlslProg;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Transform {
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

impl Transform {
    fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    fn set_position(&mut self, v: glm::Vec3) -> Transform {
        self.position = v;
        *self
    }

    fn set_rotation(&mut self, v: glm::Vec3) -> Transform {
        self.rotation = v;
        *self
    }

    fn set_scale(&mut self, v: glm::Vec3) -> Transform {
        self.scale = v;
        *self
    }
}

fn get_world_matrix(node: NodeId, arena: &Arena<Transform>) -> glm::Mat4 {
    let mut world_matrix = glm::Mat4::identity();

    for n in node.ancestors(&arena) {
        let transform = arena.get(n).unwrap().get();
        let mut model_matrix = glm::Mat4::identity();

        model_matrix = glm::translate(&model_matrix, &transform.position);

        model_matrix = glm::rotate_z(&model_matrix, transform.rotation.z);
        // model_matrix = glm::scale(&model_matrix, &transform.scale);
        world_matrix = model_matrix * world_matrix;
    }

    world_matrix
}


struct FrameData {
    transforms_arena : Arena<Transform>,
    node_a : NodeId,
    node_b : NodeId,
    node_c : NodeId,

    shader : glh::GlslProg,
    vao : glh::Vao,
}

fn setup_fn(app : &mut piralib::app::App) -> FrameData {

    let mut arena = Arena::new();

    let a = arena.new_node(Transform::new());
    let b = arena.new_node(
        Transform::new()
            .set_position(glm::vec3(250.0, 0.0, 0.0))
            .set_rotation(glm::vec3(0.0, 0.0, 0.0))
            .set_scale(glm::vec3(0.9, 0.9, 1.1)),
    );

    let c = arena.new_node(
        Transform::new()
            .set_position(glm::vec3(400.0, 0.0, 0.0))
            .set_rotation(glm::vec3(0.0, 0.0, 0.0))
            .set_scale(glm::vec3(0.9, 0.9, 1.1)),
    );

    {
        let node = arena.get_mut(a).unwrap().get_mut();
        node.position = glm::vec3(1280.0, 720.0, 0.0);
        node.rotation = glm::vec3(0.0, 0.0, 3.14 / 4.0);
        node.scale = glm::vec3(1.0, 1.0, 1.0);
    }

    a.append(b, &mut arena);
    b.append(c, &mut arena);


    let geo_rect = piralib::gl_helper::geo::Geometry::rect(-100.0, -100.0, 200.0, 200.0);
    let shader = glh::stock_shader::StockShader::new().color().build(&app.gl);
    let vao = glh::Vao::new_from_attrib(&app.gl, &geo_rect.attribs, &shader).unwrap();


    FrameData {
        transforms_arena : arena,
        node_a : a,
        node_b : b,
        node_c : c,
        vao,
        shader,
    }
}


fn update_fn(app : &mut piralib::app::App, data : &mut FrameData, egui : &egui::CtxRef){


    let gl = &app.gl;

    let FrameData{vao, shader, transforms_arena, ..} = data;

    // glh::set_viewport(gl, x, y, width, height)
    glh::set_viewport(gl, 0, 0, app.input_state.window_size.0 as i32 * 2, app.input_state.window_size.1 as i32 * 2);
    glh::clear(gl, 0.3, 0.1, 0.13, 1.0);

    {
        let p =  transforms_arena.get_mut(data.node_a).unwrap().get_mut();
        p.rotation.z = app.frame_number as f32 * 0.01; 
    }
    {
        let p =  transforms_arena.get_mut(data.node_b).unwrap().get_mut();
        p.rotation.z = app.frame_number as f32 * 0.01; 
    }

    for node in transforms_arena.iter() {
    
        let node_id = transforms_arena.get_node_id(node).unwrap();

        shader.bind(gl);
        shader.set_orthographic_matrix(gl, [app.input_state.window_size.0 as f32 *2.0, app.input_state.window_size.1 as f32 * 2.0]);
        shader.set_view_matrix(gl, &glm::Mat4::identity());


        let model_matrix = get_world_matrix(node_id, &transforms_arena);
        shader.set_model_matrix(gl, &model_matrix);

        shader.set_color(gl, &[1.0, 1.0, 0.0, 1.0]);

        vao.draw(gl, glow::TRIANGLES);
        shader.unbind(gl);
    }
   
}


fn main() {
 
    piralib::app::AppBuilder::new( piralib::app::AppSettings{
     window_title : "transforms",
     window_size : (1280, 720),
    }, setup_fn).run(update_fn);
    /*
    app.run_fn(move |event, should_quit| {
        glh::clear(0.2, 0.1, 0.1, 1.0);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::DEPTH_TEST);
        }

        {
            let transform = arena.get_mut(c).unwrap().get_mut();
            transform.position =
                glm::vec3(400.0, (event.frame_number as f32 * 0.06).sin() * 100.0, 0.0);
        }

        shader.bind();
        shader.set_uniform_mat4(
            glh::StockShader::uniform_name_perspective_matrix(),
            &glm::ortho(
                0.0,
                event.framebuffer_size.0 as f32 * 0.5,
                event.framebuffer_size.1 as f32 * 0.5,
                0.0,
                0.0,
                1.0,
            ),
        );

        shader.set_uniform_mat4(
            glh::StockShader::uniform_name_view_matrix(),
            &glm::Mat4::identity(),
        );

        {
            let model_view = get_world_matrix(a, &arena);
            shader.set_uniform_mat4(glh::StockShader::uniform_name_model_matrix(), &model_view);
            shader.set_uniform_4f(
                glh::StockShader::uniform_name_color(),
                &glm::vec4(1.0, 0.0, 0.0, 1.0),
            );
            vao.draw(gl::TRIANGLES);
        }

        {
            let model_view = get_world_matrix(b, &arena);
            shader.set_uniform_mat4(glh::StockShader::uniform_name_model_matrix(), &model_view);
            shader.set_uniform_4f(
                glh::StockShader::uniform_name_color(),
                &glm::vec4(1.0, 1.0, 1.0, 1.0),
            );
            vao.draw(gl::TRIANGLES);
        }

        {
            let model_view = get_world_matrix(c, &arena);
            shader.set_uniform_mat4(glh::StockShader::uniform_name_model_matrix(), &model_view);
            vao.draw(gl::TRIANGLES);
        }

        shader.unbind();

        let ui = event.ui;
        {
            use std::convert::TryInto;
            let node = arena.get_mut(a).unwrap().get_mut();
            ui.drag_float3(
                im_str!("Translation"),
                node.position.as_mut_slice().try_into().unwrap(),
            )
            .build();
            ui.drag_float3(
                im_str!("Scale"),
                node.scale.as_mut_slice().try_into().unwrap(),
            )
            .speed(0.01)
            .build();
            ui.drag_float(im_str!("Rotation"), &mut node.rotation.z)
                .speed(0.01)
                .build();
        }

        if cfg!(test) {
            if event.frame_number > 10 {
                let img = event.get_frame_image();
                let img = image::imageops::flip_vertical(&img);
                img.save("test_images/scene_graph.png").unwrap();
                *should_quit = true;
            }
        }
    });
    */
}

#[test]
fn scene_graph_test() {
    main();
}
