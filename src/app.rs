use egui_glow;
use glow::HasContext;

#[cfg(target_arch = "wasm32")]
pub type Event<'a, T> = winit::event::Event<'a, T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Event<'a, T> = glutin::event::Event<'a, T>;

type SetupFn<T> = fn(&mut App) -> T;
type UpdateFn<T> = fn(&mut App, &mut T, &Event<()>, &egui::CtxRef);

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub window_size : (i32, i32),
    pub window_title : &'static str,
}
pub struct AppBuilder<T : 'static> {
    setup_fn : SetupFn<T>,
    update_fn : Option<UpdateFn<T>>,
    settings : AppSettings,
}

impl<T> AppBuilder<T>{
    pub fn new(settings : AppSettings, setup_fn : SetupFn<T>) -> Self {
        Self{
            settings,
            setup_fn,
            update_fn :  None,
        }
    }

    pub fn run(mut self, update_fn : UpdateFn<T>){
        self.update_fn = Some(update_fn);

        #[cfg(not(target_arch = "wasm32"))]
        main_loop_glutin(self);

        #[cfg(target_arch = "wasm32")]
        main_loop_wasm(self);
    }
}

pub struct InputState {
   pub mouse_pos : (f32, f32),
}

pub struct App {
    pub gl : glow::Context,
    pub frame_number : u64,
    pub input_state : InputState,
    pub settings : AppSettings,

    // pub egui : egui_glow::EguiGlow,
    // pub run_ui: Option<&'a dyn FnMut(&egui::CtxRef)>,

    // #[cfg(not(target_arch = "wasm32"))]
    // pub window : Option<glutin::ContextWrapper< glutin::PossiblyCurrent, glutin::window::Window>>,
}


#[cfg(target_arch = "wasm32")]
fn main_loop_wasm<T : 'static>(builder : AppBuilder<T>){

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
        (gl, "#version 300 es")
    };
    
    let mut app  = App{
        gl, 
        settings,
        frame_number : 0,
        input_state : InputState { mouse_pos: (0.0, 0.0) },
    };

    let mut data = (builder.setup_fn)(&mut app);

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
fn main_loop_glutin<T : 'static>(builder : AppBuilder<T>){

    let settings = builder.settings.clone();
    let (gl, window, event_loop) = unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(settings.window_title)
            .with_inner_size(glutin::dpi::LogicalSize::new(settings.window_size.0, settings.window_size.1));
        
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        gl.enable(glow::FRAMEBUFFER_SRGB);

        (gl, Some(window), event_loop)
    };

    let mut egui = egui_glow::EguiGlow::new(&window.as_ref().unwrap(), &gl);

    let mut app  = App{
        gl, 
        settings,
        // window,
        // egui : egui_glow,
        // run_ui : None,
        frame_number : 0,
        input_state : InputState { mouse_pos: (0.0, 0.0) },
    };
    let mut clear_color = [0.0, 0.0, 0.0];
    
    let mut data = (builder.setup_fn)(&mut app);
    event_loop.run( move |main_event, _, control_flow| { 
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match main_event {
            glutin::event::Event::WindowEvent { ref event, .. } => match event {
                glutin::event::WindowEvent::Resized(physical_size) => window.as_ref().unwrap().resize(physical_size.clone()),
                glutin::event::WindowEvent::CloseRequested => *control_flow =  glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::CursorMoved{position, ..} =>  {
                    let scale_factor = 0.5;
                    app.input_state.mouse_pos = (position.x as f32 * scale_factor, position.y as f32 * scale_factor );
                    egui.on_event(&event);
                }
                _ => {
                    egui.on_event(&event);
                },
            },
            Event::RedrawRequested(_) => {
            },
            Event::MainEventsCleared => {
                app.frame_number = app.frame_number + 1;
                
                let (_needs_repaint, shapes) = egui.run(window.as_ref().unwrap().window(), |ui|{
                    builder.update_fn.unwrap()(&mut app, &mut data, &main_event, ui);  
                });

                egui.paint(window.as_ref().unwrap(), &app.gl, shapes);
                window.as_ref().unwrap().swap_buffers().unwrap();
            }
           _ => ()
       }
    });
}