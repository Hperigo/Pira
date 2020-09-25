use glfw::{Action, Context, Key, SwapInterval}; 
use std::sync::mpsc::Receiver;

use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

use std::time::Instant;

pub struct App {

    pub window : glfw::Window,
    pub events : Receiver<(f64, glfw::WindowEvent)>,
    pub should_quit : bool,

    pub glfw_context : glfw::Glfw,
    pub mouse_pos : glm::Vec2,


    pub imgui_glfw : ImguiGLFW,
    pub imgui : ImContext,
    last_frame_time : Instant,
}

pub struct Options {
   pub window_width : u32,
   pub window_height : u32,
   pub title: String
}

impl App{

    pub fn init_with_options(opt : &Options) -> App{
        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(opt.window_width, opt.window_height, &opt.title, glfw::WindowMode::Windowed).expect("Failed to create window");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_all_polling(true);
    //    window.make_current();


        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        glfw.set_swap_interval( SwapInterval::Sync(1) );

        let mut imgui = ImContext::create();
        let imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

        App{
            mouse_pos : glm::Vec2::new( (opt.window_width as f32 ) / 2.0 , (opt.window_height as f32) / 2.0),
            window : window,
            should_quit : false,
            events : events,
            glfw_context : glfw,
         
            imgui_glfw : imgui_glfw,
            imgui : imgui,
            last_frame_time : Instant::now(),
        }
    }

    pub fn update(&mut self){

        self.begin_ui();
        self.handle_events();


        self.window.swap_buffers();


        self.should_quit = self.window.should_close();

    }

    pub fn begin_ui(&mut self) {
        let io = self.imgui.io_mut();

        let now = Instant::now();
        let delta = now - self.last_frame_time;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame_time = now;
        io.delta_time = delta_s;

        let window_size = self.window.get_size();
        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [2.0, 2.0];
        
    }

    pub fn do_ui(&mut self, f : impl FnOnce( &imgui_glfw_rs::imgui::Ui ) ){
        let ui = self.imgui.frame();
        f( &ui );

        self.imgui_glfw.draw(ui, &mut self.window);
    }

    pub fn end_ui(&mut self) {
        
    }

    pub fn run(&mut self) -> bool {
        self.update();
        !self.should_quit
    }
    
    pub fn get_framebuffer_size(&self) -> (i32, i32){
        self.window.get_framebuffer_size()
    }

    pub fn get_window_size(&self) -> (i32, i32){
        self.window.get_size()
    }

    fn handle_events(&mut self){
        self.glfw_context.poll_events();
        for( _, event) in glfw::flush_messages(&self.events){
            self.imgui_glfw.handle_event(&mut self.imgui, &event);
            
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos = glm::Vec2::new( x as f32, y as f32 );
                }

                glfw::WindowEvent::Key( Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }

                glfw::WindowEvent::Close => {
                    self.window.set_should_close(true)
                }
                _ =>{}
            }
        }

    }
}
