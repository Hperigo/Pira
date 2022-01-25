extern crate piralib;
use piralib::app;

use piralib::gl_helper as glh;
struct FrameData {
    clear_color: [f32; 3],
}

fn m_setup(_app: &mut app::App) -> FrameData {
    FrameData {
        clear_color: [1.0, 0.0, 0.4],
    }
}

fn m_update(app: &mut app::App, _data: &mut FrameData, ui: &egui::CtxRef) {
    let gl = &app.gl;

    glh::clear(
        gl,
        _data.clear_color[0],
        _data.clear_color[1],
        _data.clear_color[2],
        1.0,
    );
    glh::set_viewport(
        gl,
        0,
        0,
        app.input_state.window_size.0 * app.get_dpi_factor() as i32,
        app.input_state.window_size.1 * app.get_dpi_factor() as i32,
    );

    egui::SidePanel::new(egui::panel::Side::Left, "panel").show(ui, |ui| {
        ui.heading("Hello World!");
        ui.color_edit_button_rgb(&mut _data.clear_color);
    });
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (500, 500),
            window_title: "simple app",
        },
        m_setup,
    )
    .run(m_update);
}
