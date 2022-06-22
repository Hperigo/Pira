extern crate piralib;

use nalgebra_glm as glm;
use piralib::app;
use piralib::event;
use piralib::gl_helper as glh;
use piralib::utils::transform_system::*;

use piralib::utils::geo;
use piralib::utils::geo::Geometry;
struct FrameData {
    transforms_arena: TransformSystem, //Arena<Transform>,
    node_a: NodeId,
    node_b: NodeId,
    node_c: NodeId,

    shader: glh::GlslProg,
    vao: glh::VaoSliced,
}

fn setup_fn(app: &mut piralib::app::App) -> FrameData {
    let mut ts = TransformSystem::new();
    let (aa, _) = ts.new_transform();
    let (bb, _) = ts.new_transform();
    let (cc, _) = ts.new_transform();

    ts.set_position(
        aa,
        glm::vec3(
            app.input_state.window_size.0 as f32 / 2.0,
            app.input_state.window_size.1 as f32 / 2.0,
            0.0,
        ),
    );
    ts.set_rotation(aa, glm::vec3(0.0, 0.0, 3.14 / 10.0));
    ts.set_scale(aa, glm::vec3(0.5, 0.5, 1.0));

    ts.set_position(bb, glm::vec3(250.0, 0.0, 0.0));
    ts.set_rotation(bb, glm::vec3(0.0, 0.0, 3.14 / 5.0));
    ts.set_scale(bb, glm::vec3(0.5, 0.5, 1.0));

    ts.set_position(cc, glm::vec3(400.0, 0.0, 0.0));
    ts.set_rotation(cc, glm::vec3(0.0, 0.0, 0.0));
    ts.set_scale(cc, glm::vec3(1.0, 1.0, 1.0));

    ts.set_parent(bb, aa, false);
    ts.set_parent(cc, bb, false);

    let (vao, shader) = geo::Rect::new(-100.0, -100.0, 200.0, 200.0).get_vao_and_shader(&app.gl); //Geometry::rect(-100.0, -100.0, 200.0, 200.0, false);

    FrameData {
        transforms_arena: ts,
        node_a: aa,
        node_b: bb,
        node_c: cc,
        vao,
        shader,
    }
}

fn event_fn(_app: &mut app::App, _data: &mut FrameData, event: &event::WindowEvent) {
    if let event::WindowEvent::MouseInput { state, .. } = event {
        if matches!(state, event::ElementState::Pressed) {}
    }

    if let event::WindowEvent::CursorMoved { position: _, .. } = event {}

    if let event::WindowEvent::KeyboardInput { input, .. } = event {
        if matches!(input.state, event::ElementState::Released) {
            if _data.transforms_arena.has_parent(_data.node_b) {
                _data.transforms_arena.clear_parent(_data.node_b, true);
            } else {
                _data
                    .transforms_arena
                    .set_parent(_data.node_b, _data.node_a, true);
            }
        }
    }
}

fn update_fn(app: &piralib::app::App, data: &mut FrameData, _egui: &piralib::egui::Context) {
    let gl = &app.gl;

    let FrameData {
        vao,
        shader,
        transforms_arena,
        ..
    } = data;
    glh::set_viewport(
        gl,
        0,
        0,
        app.input_state.window_size.0 as i32,
        app.input_state.window_size.1 as i32,
    );
    glh::clear(gl, 0.3, 0.1, 0.13, 1.0);

    if transforms_arena.has_parent(data.node_b) == false {
        transforms_arena.set_position(
            data.node_b,
            glm::vec3(
                app.input_state.mouse_pos.0 as f32,
                app.input_state.mouse_pos.1 as f32,
                0.0,
            ),
        );
    }

    // transform node a ----
    transforms_arena.set_rotation(
        data.node_a,
        glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.005),
    );
    let s = (app.frame_number as f32 * 0.01).sin();
    transforms_arena.set_scale(data.node_a, glm::vec3(s, s, s));

    // transform node b -----
    let r = transforms_arena.get_rotation(data.node_b).z + 0.01;
    transforms_arena.set_rotation(data.node_b, glm::vec3(0.0, 0.0, r));
    transforms_arena.set_rotation(
        data.node_c,
        glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.005),
    );

    for id in transforms_arena.keys() {
        shader.bind(gl);
        shader.set_orthographic_matrix(
            gl,
            [
                app.input_state.window_size.0 as f32,
                app.input_state.window_size.1 as f32,
            ],
        );
        shader.set_view_matrix(gl, &glm::Mat4::identity());

        let model_matrix = transforms_arena.get_world_matrix(*id); // get_world_matrix(node_id, &transforms_arena);
        shader.set_model_matrix(gl, &model_matrix);

        shader.set_color(gl, &[1.0, 1.0, 0.0, 1.0]);
        vao.draw(gl);

        let pos = transforms_arena.get_world_position(*id);
        shader.set_transform(
            gl,
            &pos,
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.05, 0.05, 0.05),
        );

        shader.set_color(gl, &[0.0, 0.0, 1.0, 1.0]);
        vao.draw(gl);

        shader.unbind(gl);
    }
}

fn main() {
    piralib::app::AppBuilder::new(
        piralib::app::AppSettings {
            window_title: "transforms",
            window_size: (1280, 720),
        },
        setup_fn,
    )
    .event(event_fn)
    .run(update_fn);
}

#[test]
fn scene_graph_test() {
    main();
}
