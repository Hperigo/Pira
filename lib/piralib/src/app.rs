use glfw::{Action, Context, Key, SwapInterval}; 
use std::sync::mpsc::Receiver;

pub struct App {

    pub window : glfw::Window,
    pub events : Receiver<(f64, glfw::WindowEvent)>,
    pub should_quit : bool,

    pub glfw_context : glfw::Glfw,
    pub mouse_pos : glm::Vec2
}

pub struct Options {
   pub window_width : u32,
   pub window_height : u32,
   pub title: String
}

impl<'a> App{

    pub fn init_with_options(opt : &Options) -> App {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(opt.window_width, opt.window_height, &opt.title, glfw::WindowMode::Windowed).expect("Failed to create window");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        glfw.set_swap_interval( SwapInterval::Sync(1) );

        App{
            mouse_pos : glm::Vec2::new( (opt.window_width as f32 ) / 2.0 , (opt.window_height as f32) / 2.0),
            window : window,
            should_quit : false,
            events : events,
            glfw_context : glfw,
        }

    }

    pub fn init() -> App {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(800, 600, "", glfw::WindowMode::Windowed).expect("Failed to create window");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        glfw.set_swap_interval( SwapInterval::Sync(1) );
        App{
            mouse_pos : glm::Vec2::new(0.0, 0.0),
            window : window,
            should_quit : false,
            events : events,
            glfw_context : glfw,
        }

    }

    pub fn update(&mut self){

        self.glfw_context.poll_events();
        self.handle_events();

        // self.window.gl_swap_window();
        self.window.swap_buffers();
        self.should_quit = self.window.should_close();
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
        for( _, event) in glfw::flush_messages(&self.events){
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
