extern crate piralib;
use glow::*;
use piralib::app;

struct FrameData {}

fn m_setup(_app: &mut app::App) -> FrameData {
    FrameData {}
}

<<<<<<< HEAD
fn m_update(app: &mut app::App, _data: &mut FrameData) {
=======
fn m_update(
    app: &mut app::App,
    _data: &mut FrameData,
    _event: &app::Event<()>,
    _ui: &egui::CtxRef,
) {
>>>>>>> egui
    unsafe {
        app.gl.clear(glow::COLOR_BUFFER_BIT);
        app.gl.clear_color(1.0, 0.0, 0.4, 1.0);
    }
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (200, 200),
            window_title: "simple app",
        },
        m_setup,
    )
    .run(m_update);
}
