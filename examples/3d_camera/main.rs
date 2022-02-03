extern crate piralib;

use glow::HasContext;
use piralib::glow;
use piralib::gl_helper as glh;
use piralib::nalgebra_glm as glm;
use piralib::app;

use piralib::event;

use piralib::utils::camera::{OrbitCamera, Camera};

struct FrameData {
    shader : glh::GlslProg,
    vao : glh::Vao,

    cube_shader : glh::GlslProg,
    cube_vao : glh::Vao,
    camera : OrbitCamera,
}

fn m_setup(app: &mut app::App) -> FrameData {

    let gl = &app.gl;

    let geo =  glh::geo::Geometry::axis(2.0); //glh::geo::Geometry::circle(0.0, 0.0, 1.0, false);
    let shader = glh::StockShader::new().color().build(gl);
    let vao = glh::Vao::new_from_attrib(gl, &geo.attribs, glow::LINES, &shader).unwrap();

    let geo =  glh::geo::Geometry::cube(0.5); //glh::geo::Geometry::circle(0.0, 0.0, 1.0, false);
    let cube_shader = glh::StockShader::new().build(gl);
    let cube_vao = glh::Vao::new_from_attrib(gl, &geo.attribs, glow::TRIANGLES, &shader).unwrap();

    let aspect_ratio = app.input_state.window_size.0 as f32 / app.input_state.window_size.1 as f32;

    let camera = OrbitCamera::new( aspect_ratio, 45.0, 0.0001, 1000.0 );
    
    FrameData {
        cube_vao,
        cube_shader,
        vao,
        shader,
        camera,
    }
}

fn m_event( _app : &mut app::App, _data : &mut FrameData, event : &event::WindowEvent ){

    _data.camera.handle_event(event, _app);
    // if let event::WindowEvent::KeyboardInput { .. } = event {
    // }
}

fn m_update(
    app: &mut app::App,
    data: &mut FrameData,
    _ui: &egui::CtxRef,
) {
    let gl = &app.gl;
    let axis_shader = &data.shader;
    let axis_vao = &data.vao;

    let cube_shader = &data.cube_shader;
    let cube_vao = &data.cube_vao;

    let camera = &mut data.camera;


    egui::SidePanel::new( egui::panel::Side::Left, "camera settings").show(_ui, |_ui| {

   });

   camera.on_resize(app.get_window_size()[0], app.get_window_size()[1]);
    camera.update();
    //let target_id = camera.target;
    //camera.transforms.set_rotation( &target_id, glm::vec3( 0.0, app.frame_number as f32 * 0.001,  0.0) );

    let persp_matrix = camera.get_perspective_matrix();
    let view_matrix = camera.get_view_matrix();


    glh::clear(gl,0.8, 0.8, 0.8, 1.0);

    cube_shader.bind(gl);

    unsafe{ 
        gl.cull_face(glow::FRONT_AND_BACK);
        gl.enable(glow::DEPTH_TEST);
    }

    cube_shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(), &persp_matrix);
    cube_shader.set_view_matrix(gl, &view_matrix);

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3(0.0, 0.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(1.0, 1.0, 1.0));
    
    cube_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    cube_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0] );

    cube_vao.draw(gl);

    cube_shader.unbind(gl);

    axis_shader.bind(gl);
    let target_t = camera.get_target_world_matrix(); //transforms.get_world_matrix(camera.target);
    axis_shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(), &persp_matrix);
    axis_shader.set_view_matrix(gl, &view_matrix);
    axis_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &target_t );
    axis_vao.draw(gl);
    
    axis_shader.unbind(gl);

    unsafe{
        gl.disable(glow::DEPTH_TEST);
    }
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1920, 1080),
            window_title: "simple app",
        },
        m_setup,
    )
    .event(m_event)
    .run(m_update);
}
