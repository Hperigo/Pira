extern crate piralib;
use glow::*;
use nalgebra_glm as glm;
use piralib::app;
use piralib::gl_helper as glh;
use rand::Rng;

struct FrameData {
    vao: glh::Vao,
    shader: glh::GlslProg,
    time: f32,
    number_of_instances: i32,

    base_color: [f32; 3],
    tip_color: [f32; 3],
}

fn m_setup(app: &mut app::App) -> FrameData {
    let gl = &app.gl;

    #[cfg(not(target_arch = "wasm32"))]
    let shader_version = "#version 400";

    #[cfg(target_arch = "wasm32")]
    let shader_version = "#version 300 es";

    let vertex_shader_string = format!("{}
    precision highp float;

    uniform mat4 uModelMatrix;
    uniform mat4 uPerspectiveMatrix;
    uniform mat4 uViewMatrix;
    uniform float uTime;
    
    uniform vec2 uMousePos;

    in vec3 inPosition;
    in vec4 inColor; 
    in vec2 instancePosition;

    out vec4 vColor;
    void main()
    {{   

        float dist = (distance(uMousePos, instancePosition.xy) / 50.0);

        float angle =  float(gl_InstanceID) * 0.0001 + instancePosition.y * 0.007;
        angle = (angle + dist) + uTime * -0.01 ;
        mat3  rotation = mat3(
            vec3( cos(angle), -sin(angle), 0.0),
            vec3( sin(angle), cos(angle),  0.0),
            vec3( 0.0,        0.0,         1.0 ));
        

        float fur_length = (sin(angle) + 1.0) / 2.0;
        vec3 pos = inPosition;
        pos.x = (pos.x - 5.0 )* ( 1.0 -  inColor.r + 0.0 ) ;
        pos.z = pos.z + 1.0 * inColor.r;
        vec3 rotatedPoint =  rotation * vec3(pos * (fur_length + 0.5)) + vec3(instancePosition, 0.0);

        gl_Position = uPerspectiveMatrix * uViewMatrix * uModelMatrix * vec4(rotatedPoint, 1.0);
        
        vColor = inColor;
    }}   
    ", shader_version);

    let frag_shader_string = format!(
        "{}
    precision highp float;

    in vec4 vColor;

    uniform vec3 uTipColor;
    uniform vec3 uBaseColor;

    out vec4 Color;
    void main()
    {{   
        float alpha = vColor.g;
        Color = vec4(mix(uBaseColor, uTipColor, vColor.g), 1.0);
    }}
    ",
        shader_version
    );

    let mut pos_attrib = glh::VertexAttrib::new_position_attr();

    // build vertex data ----
    let fur_width = 10.0;

    let mut vertices: Vec<f32> = Vec::new();
    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![fur_width, 30.0, 0.0]);
    vertices.append(&mut vec![0.0, 30.0, 0.0]);

    vertices.append(&mut vec![0.0, 0.0, 0.0]);
    vertices.append(&mut vec![fur_width, 30.0, 0.0]);
    vertices.append(&mut vec![fur_width, 0.0, 0.0]);

    let mut color_attrib = glh::VertexAttrib::new_color_attr();
    let mut colors: Vec<f32> = Vec::new();
    colors.append(&mut vec![0.0, 0.1, 0.1, 1.0]);
    colors.append(&mut vec![1.0, 0.9, 0.1, 1.0]);
    colors.append(&mut vec![1.0, 0.9, 0.1, 1.0]);

    colors.append(&mut vec![0.0, 0.1, 0.1, 1.0]);
    colors.append(&mut vec![1.0, 0.9, 0.1, 1.0]);
    colors.append(&mut vec![0.0, 0.1, 0.1, 1.0]);

    //create the instance position attribute buffer
    let mut instance_positions: Vec<f32> = Vec::new();
    let spacing = 10.;
    let mut rng = rand::thread_rng();

    let random_range = 1.0;

    #[cfg(target_arch="wasm32")]
    let max_x = 550 / 2;
    #[cfg(target_arch="wasm32")]
    let max_y = 190 / 2;

    #[cfg(not(target_arch="wasm32"))]
    let max_x = 550;
    #[cfg(not(target_arch="wasm32"))]
    let max_y = 190;

    // let max_x = 550 / 2;
    // let max_y = 190 / 2;
    for i in 0..max_x {
        for k in 0..max_y {
            let x = ((max_x as f32) - (i as f32)) + rng.gen_range(-random_range..random_range);
            let y = ((max_y as f32) - (k as f32)) + rng.gen_range(-random_range..random_range);
            instance_positions.append(&mut vec![x as f32 * spacing * 0.5, y as f32 * spacing]);
        }
    }

    let number_of_instances = instance_positions.len() as i32 / 2;
    println!("number of instances: {}", number_of_instances);

    let instance_positions_attrib = glh::VertexAttrib {
        name: "instancePosition",
        size: 2,
        stride: 0,
        data: instance_positions,
        per_instance: true,
    };

    pos_attrib.data = vertices;
    color_attrib.data = colors;

    let shader = glh::GlslProg::new(
        gl,
        vertex_shader_string.as_str(),
        frag_shader_string.as_str(),
    );
    let attribs = vec![pos_attrib, color_attrib, instance_positions_attrib];
    let vao = glh::Vao::new_from_attrib(gl, &attribs, &shader).unwrap();

    FrameData {
        vao,
        shader,
        number_of_instances,
        time: 0.0,

        base_color: [0.2, 0.1, 0.1],
        tip_color: [0.9, 0.0, 0.2],
    }
}

fn m_update(
    app: &mut app::App,
    _data: &mut FrameData,
    ui: &piralib::egui::CtxRef,
) {
    let time = &mut _data.time;
    let gl = &app.gl;
    let shader = &_data.shader;
    let vao = &_data.vao;

    let mut mouse_pos: [f32; 2] = [0.0, 0.0];

    let mut base_color = &mut _data.base_color;
    let mut tip_color = &mut _data.tip_color;

    *time = *time + 1f32;

    mouse_pos[0] = app.input_state.mouse_pos.0; //mouse_pos[0] + ((app.input_state.mouse_pos.0 * 1.0) - mouse_pos[0]) * 1.0;
    mouse_pos[1] = app.input_state.mouse_pos.1; //mouse_pos[1] + ((app.input_state.mouse_pos.1 * 1.0) - mouse_pos[1]) * 1.0;

    #[cfg(not(target_arch="wasm32"))]
    egui::SidePanel::new(egui::panel::Side::Left, "panel").show(ui, |ui| {
        ui.label("base color");
        ui.color_edit_button_rgb(&mut base_color);
        
        ui.label("tip color");
        ui.color_edit_button_rgb(&mut tip_color);
        //ui.color_edit_button_rgb(&mut tip_color);
    });

    let scale_factor = app.get_dpi_factor();
    unsafe{
        gl.disable(glow::FRAMEBUFFER_SRGB);
        gl.clear_color(base_color[0], base_color[1], base_color[2], 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        gl.enable(glow::DEPTH_TEST);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
<<<<<<< HEAD
        gl.viewport(0, 0, app.input_state.window_size.0 * scale_factor as i32, app.input_state.window_size.1  * scale_factor as i32);
=======
        gl.viewport(0, 0, app.input_state.window_size.0 * 1, app.input_state.window_size.1 * 1);
>>>>>>> bd63c97a541b62c6481dcb1d8be5388bb189afbf
    }

    shader.bind(gl);

    shader.set_uniform_mat4(
        gl,
        glh::StockShader::uniform_name_perspective_matrix(),
        &glm::ortho(
            0.0,
            app.input_state.window_size.0 as f32 * scale_factor, // beacuse of mac dpi we need to scale it down
            app.input_state.window_size.1 as f32 * scale_factor,
            0.0,
            -1.0,
            1.0,
        ),
    );

    shader.set_uniform_1f(gl, "uTime", *time);
    shader.set_uniform_2f(gl, "uMousePos", &mouse_pos);

    shader.set_uniform_3f(gl, "uBaseColor", &base_color);
    shader.set_uniform_3f(gl, "uTipColor", &tip_color);

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

    vao.draw_instanced(gl, glow::TRIANGLES, _data.number_of_instances);

    shader.unbind(gl);


    unsafe{
        gl.disable( glow::DEPTH_TEST );
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
    .run(m_update);
}
