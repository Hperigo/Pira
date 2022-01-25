use egui::Window;
use glutin::PossiblyCurrent;
#[cfg(target_arch = "wasm32")]
use winit::event;

#[cfg(not(target_arch = "wasm32"))]
use glutin::event;

#[cfg(not(target_arch = "wasm32"))]
pub use egui::CtxRef;

#[cfg(not(target_arch = "wasm32"))]
use egui_glow;

#[cfg(target_arch = "wasm32")]
pub mod egui {
    #[derive(Default)]
    pub struct CtxRef{}    
}

pub type Event<'a, T> = event::Event<'a, T>;
type SetupFn<T> = fn(&mut App) -> T;
type UpdateFn<T> = fn(&mut App, &mut T, ui : &egui::CtxRef);
type EventFn<T> = fn(&mut App, &mut T, &event::WindowEvent);

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub window_size: (i32, i32),
    pub window_title: &'static str,
}
pub struct AppBuilder<T: 'static> {
    setup_fn: SetupFn<T>,
    update_fn: Option<UpdateFn<T>>,
    event_fn: Option<EventFn<T>>,
    settings: AppSettings,
}

impl<T> AppBuilder<T> {
    pub fn new(settings: AppSettings, setup_fn: SetupFn<T>) -> Self {
        Self {
            settings,
            setup_fn,
            update_fn: None,
            event_fn : None,
        }
    }

    pub fn event(mut self, event_fn: EventFn<T>) -> Self {
        self.event_fn = Some(event_fn);
        self
    }

    pub fn run(mut self, update_fn: UpdateFn<T>) {
        self.update_fn = Some(update_fn);

        #[cfg(not(target_arch = "wasm32"))]
        main_loop_glutin(self);

        #[cfg(target_arch = "wasm32")]
        main_loop_wasm(self);
    }
}

pub struct InputState {
    pub window_size : (i32, i32),
    pub window_pos :  (i32, i32),
    
    pub mouse_pos: (f32, f32),
}

pub struct App {
    pub gl: glow::Context,
    pub context : glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window >,

    pub frame_number: u64,
    pub input_state: InputState,
}

impl App {
    pub fn get_dpi_factor(&self) -> f32 {
        self.context. window().scale_factor() as f32
    }
}


#[cfg(target_arch = "wasm32")]
fn main_loop_wasm<T: 'static>(builder: AppBuilder<T>) {
    let event_loop = winit::event_loop::EventLoop::new();
    let settings = builder.settings.clone();


    use wasm_bindgen::JsCast;
    use winit::{platform::web::WindowBuilderExtWebSys};
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);

    let window = winit::window::WindowBuilder::new()
    .with_title("A fantastic window!")
    .with_canvas(Some(canvas))
    .build(&event_loop)
    .unwrap();

    let mut egui  = egui::CtxRef::default();

    let mut app = App {
        gl,
        settings,
        frame_number: 0,
        input_state: InputState {
            mouse_pos: (0.0, 0.0),
            window_size : settings.window_size,
            window_pos :(0, 0),
        },
    };

    let mut data = (builder.setup_fn)(&mut app);

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    event_loop.run(move |event, _, control_flow| {
        
        match event {
            
            Event::WindowEvent { event, .. } => {
                // use winit::event::WindowEvent;
                // let raw_input = egui::RawInput::default();


                // let (_needs_repaint, shapes) = egui.run(raw_input , |egui_ctx| {

                // });

                if builder.event_fn.is_some() {
                    builder.event_fn.unwrap()(&mut app, &mut data, &event);
                }

                if matches!(event, event::WindowEvent::CloseRequested | event::WindowEvent::Destroyed) {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                if let event::WindowEvent::CursorMoved { position, .. } = event {
                        let scale_factor = 0.5;
                    app.input_state.mouse_pos = (
                        position.x as f32 * scale_factor,
                        position.y as f32 * scale_factor,
                    );
                }


                if let winit::event::WindowEvent::Resized(physical_size) = event {
                    unsafe{web_sys::console::log_1(&"resized!".into());}
                    *control_flow = winit::event_loop::ControlFlow::Wait;
                    app.input_state.window_size = (physical_size.width as i32, physical_size.height as i32);
                }
            },
            
            Event::DeviceEvent { event ,..} => {
                // if let DeviceEvent::MouseMotion{position, ..} = event {
                //     app.input_state.mouse_pos
                // }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {
            },
        }
        app.frame_number = app.frame_number + 1;
        builder.update_fn.unwrap()(&mut app, &mut data, &egui);
        // builder.update_fn.unwrap()(&mut app, &mut data, &egui);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main_loop_glutin<T: 'static>(builder: AppBuilder<T>) {
    use glutin::{event::VirtualKeyCode, window::Window};

    let settings = builder.settings;
    let (gl, window, event_loop) = unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(settings.window_title)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                settings.window_size.0,
                settings.window_size.1,
            ));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        (gl, window, event_loop)
    };

    let mut egui = egui_glow::EguiGlow::new(&window, &gl);
    
    println!("creating app...");
    let window_size = (settings.window_size.0 * window.window().scale_factor() as i32, settings.window_size.1 * window.window().scale_factor() as i32);
    let window_pos = window.window().inner_position().unwrap().into();

    let mut app = App {
        gl,
        frame_number: 0,
        context : window,
        input_state: InputState {
            window_size,
            window_pos,
            mouse_pos: (0.0, 0.0),
        },
    };

    let mut data = (builder.setup_fn)(&mut app);

    event_loop.run(move |event, _, control_flow| {

        let mut redraw = || {
            app.frame_number += 1;

            // For future versions of egui we need to use this
            let raw_input = egui.egui_winit.take_egui_input(app.context.window());
            let (_needs_repaint, shapes) =  egui.egui_ctx.run(raw_input, |egui_ctx| {
                builder.update_fn.unwrap()(&mut app, &mut data, egui_ctx);
            });

                
            // egui.begin_frame(window.window());
            // builder.update_fn.unwrap()(&mut app, &mut data,  &egui.egui_ctx);
            // let (_needs_repain, shapes) = egui.end_frame(window.window());

            // draw things behind egui here
            egui.paint(&app.context, &app.gl, shapes);

            // draw things on top of egui here
        
            app.context.swap_buffers().unwrap();
            app.context.window().request_redraw();
        };

        match event {
            glutin::event::Event::RedrawRequested(_)  => {
                redraw() ;
                *control_flow = glutin::event_loop::ControlFlow::Poll;
            },
            glutin::event::Event::MainEventsCleared => {
                app.context.window().request_redraw();
            }
            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;

                let did_use_egui = egui.on_event(&event);

                if builder.event_fn.is_some() && !did_use_egui {
                    builder.event_fn.unwrap()(&mut app, &mut data, &event);
                }

                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = event {
                    app.context.resize(physical_size);

                    let scale_factor = app.context.window().scale_factor() as i32;
                    println!("scale factor: {}", scale_factor);
                    app.input_state.window_size.0 = physical_size.width as i32 / scale_factor;
                    app.input_state.window_size.1 = physical_size.height as i32 / scale_factor;
                    *control_flow = glutin::event_loop::ControlFlow::Wait;
                }

                if let  glutin::event::WindowEvent::Moved( _position ) = event {
                    *control_flow = glutin::event_loop::ControlFlow::Wait;
                }

                if let glutin::event::WindowEvent::CursorMoved { position, .. } = event {
                    let scale_factor = 1.0; 
                    app.input_state.mouse_pos = (
                        position.x as f32 * scale_factor,
                        position.y as f32 * scale_factor,
                    );
                }

                if let glutin::event::WindowEvent::KeyboardInput { input, .. } = event {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                }

                //window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                egui.destroy(&app.gl);
            }

            _ => (),
        }
    });
}
