extern crate piralib;

use nalgebra_glm as glm;
use piralib::gl_helper as glh;

use piralib::utils::TransformSystem::*;

 struct FrameData {
    transforms_arena :  TransformSystem, //Arena<Transform>,
    node_a : NodeId,
    node_b : NodeId,
    node_c : NodeId,

    shader : glh::GlslProg,
    vao : glh::Vao,
}

fn setup_fn(app : &mut piralib::app::App) -> FrameData {



    let mut ts = TransformSystem::new();
    let (aa, _) = ts.new_transform();
    let (bb, _) = ts.new_transform();
    let (cc, _) = ts.new_transform();

    ts.set_position(&aa,glm::vec3(app.input_state.window_size.0 as f32 / 2.0, app.input_state.window_size.1 as f32 / 2.0, 0.0));
    ts.set_rotation(&aa,glm::vec3(0.0, 0.0, 3.14 / 4.0));
    ts.set_scale(&aa, glm::vec3(1.0, 1.0, 1.0));


    ts.set_position(&bb,glm::vec3(250.0, 0.0, 0.0));
    ts.set_rotation(&bb,glm::vec3(0.0, 0.0, 0.0));
    ts.set_scale(&bb, glm::vec3(1.0, 1.0, 1.0));

    ts.set_position(&cc,glm::vec3(400.0, 0.0, 0.0));
    ts.set_rotation(&cc,glm::vec3(0.0, 0.0, 0.0));
    ts.set_scale(&cc, glm::vec3(1.0, 1.0, 1.0));

    
    ts.set_parent(&bb, Some(aa));
    ts.set_parent(&cc, Some(bb));


    let geo_rect = piralib::gl_helper::geo::Geometry::rect(-100.0, -100.0, 200.0, 200.0);
    let shader = glh::stock_shader::StockShader::new().color().build(&app.gl);
    let vao = glh::Vao::new_from_attrib(&app.gl, &geo_rect.attribs, &shader).unwrap();


    FrameData {
        transforms_arena : ts,
        node_a : aa,
        node_b : bb,
        node_c : cc,
        vao,
        shader,
    }
}


fn update_fn(app : &mut piralib::app::App, data : &mut FrameData, _egui : &piralib::app::CtxRef){


    let gl = &app.gl;

    let FrameData{vao, shader, transforms_arena, ..} = data;
    let dpi = 2.0;
    // glh::set_viewport(gl, x, y, width, height)
    glh::set_viewport(gl, 0, 0, app.input_state.window_size.0 as i32 * dpi as i32, app.input_state.window_size.1 as i32 * dpi as i32);
    glh::clear(gl, 0.3, 0.1, 0.13, 1.0);

    
    {   
        // transforms_arena.set_position(&data.node_a, glm::vec3(app.input_state.mouse_pos.0 as f32 * 2.0, app.input_state.mouse_pos.1 as f32 * 2.0, 0.0));
        transforms_arena.set_rotation(&data.node_a, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.001 ));
        let s = ((app.frame_number as f32) * 0.01).sin();
        transforms_arena.set_scale(&data.node_a, glm::vec3(s,s,s));
    }


    {
        transforms_arena.set_rotation(&data.node_b, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.001 ));
        let s = ((app.frame_number as f32) * 0.01 + 1.0).sin();
        transforms_arena.set_scale(&data.node_b, glm::vec3(s,s,s));
    }

    {
        transforms_arena.set_rotation(&data.node_c, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.001 ));
        let s = ((app.frame_number as f32) * 0.01 + 2.0).sin();
        transforms_arena.set_scale(&data.node_c, glm::vec3(s,s,s));


    }


    for id in transforms_arena.keys() {

        shader.bind(gl);
        shader.set_orthographic_matrix(gl, [app.input_state.window_size.0 as f32 * dpi, app.input_state.window_size.1 as f32 * dpi]);
        shader.set_view_matrix(gl, &glm::Mat4::identity());

        let model_matrix =   transforms_arena.get_world_matrix(id);  // get_world_matrix(node_id, &transforms_arena);
        shader.set_model_matrix(gl, &model_matrix);


        let pos = model_matrix * glm::vec4(0.0, 0.0, 0.0, 1.0);
    
        let mouse_pos =  glm::vec3(app.input_state.mouse_pos.0 * 2.0, app.input_state.mouse_pos.1 * 2.0, 0.0);

        if glm::distance( &mouse_pos, &pos.xyz() ) < 100.0 * transforms_arena.get_world_scale(id).x.abs() {
            shader.set_color(gl, &[0.0, 0.0, 1.0, 1.0]);
        }else{
            shader.set_color(gl, &[1.0, 1.0, 0.0, 1.0]);
        }

        vao.draw(gl, glow::TRIANGLES);
        shader.unbind(gl);
    }
   
}


fn main() {
 
    piralib::app::AppBuilder::new( piralib::app::AppSettings{
     window_title : "transforms",
     window_size : (1280, 720),
    }, setup_fn).run(update_fn);

}

#[test]
fn scene_graph_test() {
    main();
}
