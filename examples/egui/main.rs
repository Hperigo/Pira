
extern crate piralib;
use piralib::app;
use glow::*;

struct FrameData { 

    clear_color : [f32; 3],
}

fn m_setup( _app : &mut app::App) -> FrameData {
    FrameData{
        clear_color : [ 1.0, 0.0, 0.4]
     }
}

fn m_update(app : &mut app::App, _data : &mut FrameData, _event : &app::Event<()>)
{   
    unsafe {
        app.gl.clear( glow::COLOR_BUFFER_BIT );
        app.gl.clear_color(_data.clear_color[0], _data.clear_color[1],_data.clear_color[2], 1.0);

        app.gl.viewport(0, 0, 500, 500);
    } 
    
    egui::SidePanel::left("my_side_panel").show(app.egui.ctx(), |ui| {
        ui.heading("Hello World!");
        if ui.button("Quit").clicked() {
            println!("QUIT");
        }

        ui.color_edit_button_rgb(&mut _data.clear_color);
    });
}

fn main() {
    app::AppBuilder::new(app::AppSettings{
        window_size : (500, 500),
        window_title : "simple app",
    }, m_setup).run(m_update);
}