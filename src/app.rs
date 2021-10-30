#[cfg(target_arch = "wasm32")]
pub type Event<'a, T> = winit::event::Event<'a, T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Event<'a, T> = glutin::event::Event<'a, T>;



type SetupFn<T> = fn(&mut App) -> T;
type UpdateFn<T> = fn(&mut App, &mut T, &Event<()>);
pub struct AppBuilder<T : 'static> {
    setup_fn : SetupFn<T>,
    update_fn : Option<UpdateFn<T>>,
}

impl<T> AppBuilder<T>{
    pub fn new(setup_fn : SetupFn<T>) -> Self {
        Self{
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

#[cfg(target_arch = "wasm32")]
pub struct App {
    pub gl : glow::Context,
    pub frame_number : u64,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct App {
    pub gl : glow::Context,
    pub window : Option<glutin::ContextWrapper< glutin::PossiblyCurrent, glutin::window::Window>>,
    pub frame_number : u64,
}


#[cfg(target_arch = "wasm32")]
fn main_loop_wasm<T : 'static>(builder : AppBuilder<T>){

    let event_loop = winit::event_loop::EventLoop::new();

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
    
    let mut data = (builder.setup_fn)();
    let mut app  = App{
        gl, 
        frame_number : 0,
    };

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
    
    let (gl, window, event_loop) = unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        (gl, Some(window), event_loop)
    };
    
    let mut app  = App{
        gl, 
        window,
        frame_number : 0,
    };

    let mut data = (builder.setup_fn)(&mut app);
    event_loop.run( move |event, _, control_flow| { 

       match event {
           glutin::event::Event::WindowEvent { ref event, .. } => {

           },
           _ => ()
       }

       app.frame_number = app.frame_number + 1;
       builder.update_fn.unwrap()(&mut app, &mut data, &event);
       app.window.as_ref().unwrap().swap_buffers().unwrap();
    });
}