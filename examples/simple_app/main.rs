
extern crate piralib;
use piralib::app;
use glow::*;

struct FrameData { }

fn m_setup( app : &mut App) -> FrameData {
    FrameData{ }
}

fn m_update(app : &mut app::App, _data : &mut FrameData, _event : &app::Event<()>)
{   
    unsafe {
        app.gl.clear( glow::COLOR_BUFFER_BIT );
        app.gl.clear_color(1.0, 0.0, 0.4, 1.0);
    }    
}

fn main() {
    app::AppBuilder::new(m_setup).run(m_update);
}