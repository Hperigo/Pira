extern crate sdl2;
use std::rc::Rc;
use std::cell::RefCell;

use gl_helper as glh;

pub struct App {
    sdl_handle : sdl2::Sdl,
    sdl_video_handle : sdl2::VideoSubsystem,
    sdl_event_pump : sdl2::EventPump,
    pub window : sdl2::video::Window,
    pub should_quit : bool,

    gl_handle  : (),
    gl_context : sdl2::video::GLContext,
    pub update_fn : Option<std::rc::Rc< Fn() >>
}

pub struct Options {
   pub window_width : u32,
   pub window_height : u32,
   pub title: String
}

impl App{

    pub fn init_with_options(opt : &Options) -> RefCell<App> { 
        
        let sdl_handle = sdl2::init().unwrap();
        let sdl_video = sdl_handle.video().unwrap();   

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(4, 1);

        let window = sdl_video
            .window(&opt.title, opt.window_width, opt.window_height)
            .resizable()
            .allow_highdpi()
            .opengl()
            .build()
            .unwrap();

        let event_pump = sdl_handle.event_pump().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gll_handle = gl::load_with( |s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void );

        // initialize some default gl values... 
        glh::set_window_matrices( 0, 0, window.drawable_size().0 as i32, window.drawable_size().1 as i32 );

        RefCell::new(App{
            sdl_handle : sdl_handle,
            sdl_video_handle : sdl_video,
            sdl_event_pump : event_pump,
            window : window,
            should_quit : false,
            gl_handle : gll_handle,
            gl_context  :  gl_context,
            update_fn : None,
        })
    }

    pub fn init() -> RefCell<App> { 

        let sdl_handle = sdl2::init().unwrap();
        let sdl_video = sdl_handle.video().unwrap();   

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(4, 1);

        let window = sdl_video
            .window("pira!", 1920, 1080)
            .resizable()
            .allow_highdpi()
            .opengl()
            .build()
            .unwrap();

        let event_pump = sdl_handle.event_pump().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gll_handle = gl::load_with( |s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void );

        // initialize some default gl values... 
        glh::set_window_matrices( 0, 0, window.drawable_size().0 as i32, window.drawable_size().1 as i32 );

        RefCell::new(App{
            sdl_handle : sdl_handle,
            sdl_video_handle : sdl_video,
            sdl_event_pump : event_pump,
            window : window,
            should_quit : false,
            gl_handle : gll_handle,
            gl_context  :  gl_context,
            update_fn : None,
        })
    }


    pub fn set_update_fn(&mut self,  function : Rc<Fn()> ){
        self.update_fn = Some( function );
    }

    pub fn update(&mut self){
        for event in self.sdl_event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit { .. } => self.should_quit = true,
                _ => {},
            }
        }


        match &self.update_fn{
            Some( x ) =>{  x(); },
            None => {},
        }
        self.window.gl_swap_window();
    }

    pub fn run(&mut self) -> bool {
    
        self.update();

        !self.should_quit
    }
    
}
