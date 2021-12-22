use egui_glow;

#[cfg(target_arch = "wasm32")]
use winit::event;

#[cfg(not(target_arch = "wasm32"))]
use glutin::event;

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
    pub frame_number: u64,
    pub input_state: InputState,
    pub settings: AppSettings,
}

#[cfg(target_arch = "wasm32")]
fn main_loop_wasm<T: 'static>(builder: AppBuilder<T>) {
    let event_loop = winit::event_loop::EventLoop::new();
    let settings = builder.settings.clone();

    let window = winit::window::WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let (gl, shader_version) = {
        use wasm_bindgen::JsCast;
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
        gl
    };

    let mut app = App {
        gl,
        settings,
        frame_number: 0,
        input_state: InputState {
            mouse_pos: (0.0, 0.0),
        },
    };

    let mut data = (builder.setup_fn)(&mut app);

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = winit::event_loop::ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
        app.frame_number = app.frame_number + 1;
        builder.update_fn.unwrap()(&mut app, &mut data, &event);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main_loop_glutin<T: 'static>(builder: AppBuilder<T>) {
    use glutin::event::VirtualKeyCode;

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
    
    let mut app = App {
        gl,
        settings,
        frame_number: 0,
        input_state: InputState {
            mouse_pos: (0.0, 0.0),
            window_size : settings.window_size,
            window_pos : window.window().inner_position().unwrap().into(),
        },
    };

    let mut data = (builder.setup_fn)(&mut app);

    event_loop.run(move |event, _, control_flow| {


        let mut redraw = || {
            app.frame_number += 1;

            let (_needs_repaint, shapes) = egui.run(window.window(), |egui_ctx| {
                builder.update_fn.unwrap()(&mut app, &mut data, egui_ctx);
            });

            // draw things behind egui here
            egui.paint(&window, &app.gl, shapes);

            // draw things on top of egui here
            window.window().request_redraw();
            window.swap_buffers().unwrap();
        };

        match event {
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),
            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;

                if builder.event_fn.is_some() {
                    builder.event_fn.unwrap()(&mut app, &mut data, &event);
                }

                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = event {
                    window.resize(physical_size);
                    *control_flow = glutin::event_loop::ControlFlow::Wait;
                }

                if let  glutin::event::WindowEvent::Moved( _position ) = event {
                    *control_flow = glutin::event_loop::ControlFlow::Wait;
                }

                if let glutin::event::WindowEvent::CursorMoved { position, .. } = event {
                    let scale_factor = 0.5;
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

                egui.on_event(&event);

                //window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                egui.destroy(&app.gl);
            }

            _ => (),
        }
    });
}
