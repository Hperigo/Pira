extern crate piralib;

use glow::HasContext;
use glutin::dpi::PhysicalPosition;
use piralib::event::MouseButton;
use piralib::glow;
use piralib::gl_helper as glh;
use piralib::nalgebra_glm as glm;
use piralib::app;

use piralib::event;

use piralib::utils::transform_system;

pub trait Camera {
    fn get_view_matrix(&self) -> glm::Mat4;
    fn get_perspective_matrix(&self) -> glm::Mat4;
}

pub struct PerspCamera {
    eye : glm::Vec3,
    target : glm::Vec3,

    fov : f32,
    near : f32,
    far : f32,

    aspect_ratio : f32,
}

impl PerspCamera {
     pub fn new( aspect_ratio : f32, fov : f32, near : f32, far : f32 ) -> Self {
        Self {
            eye : glm::vec3(0.0, 0.0, 0.0),
            target : glm::vec3( 0.0, 0.0, 0.0 ),
            fov,
            near,
            far,
            aspect_ratio,
        }
    }
}

impl Camera for PerspCamera{

    fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.eye, &self.target, &glm::vec3(0.0, 1.0, 0.0))
    }

    fn get_perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective( self.aspect_ratio, self.fov, self.near, self.far)
    }
}

pub struct OrbitCamera {

    transforms : transform_system::TransformSystem,

    eye : transform_system::NodeId,
    target : transform_system::NodeId,

    lat : f32,
    lon: f32,
    distance : f32,

    fov : f32,
    near : f32,
    far : f32,

    aspect_ratio : f32,

    last_mouse_input : PhysicalPosition<f32>,
    is_left_mouse_dragging : bool,
    is_middle_mouse_dragging : bool,
}

impl OrbitCamera {
     pub fn new(aspect_ratio : f32, fov : f32, near : f32, far : f32 ) -> Self {

        let mut transforms = transform_system::TransformSystem::new();

        let (eye, _ ) = transforms.new_transform();
        let (target, _) = transforms.new_transform();

        transforms.set_parent(eye, target, true);
        transforms.set_position(eye, glm::vec3(0.0, 0.0, 10.0));
        
        Self {
            eye,
            target,
            fov,
            near,
            far,
            aspect_ratio,
            transforms,

            lat : 90.0,
            lon : 0.0,
            distance : 5.0,

            last_mouse_input : PhysicalPosition::new(0.0, 0.0),
            is_left_mouse_dragging : false,
            is_middle_mouse_dragging : false,
        }
    }

    fn mouse_pressed(&mut self, value : bool, middle_mouse : bool ){
        
        self.is_left_mouse_dragging = value;
        self.is_middle_mouse_dragging = middle_mouse;

        if value == true || middle_mouse == true{
            self.last_mouse_input.x = 0.0;
            self.last_mouse_input.y = 0.0;
        }
    }

    fn mouse_input(&mut self, pos : PhysicalPosition<f64>, window_size : [f32; 2]){

        let pos = PhysicalPosition::new( pos.x as f32 / window_size[0], pos.y as f32 / window_size[1] );

        if self.last_mouse_input.x == 0.0 {
            self.last_mouse_input.x = pos.x;
            self.last_mouse_input.y = pos.y;
        }

        if self.is_middle_mouse_dragging == true {
            
            let dx = (pos.x - self.last_mouse_input.x) * 0.9;
            let dy = (pos.y - self.last_mouse_input.y) * 0.9;

            let target_transform = self.transforms.get_transform( self.target );
            let eye_transform = self.transforms.get_transform( self.eye );

            let mut original = eye_transform.position - target_transform.position;
            original = glm::normalize( &original );
            let mut  a_vec= glm::cross( &original, &glm::vec3(0.0, 1.0, 0.0));
            a_vec = glm::normalize(&a_vec);

            let mut b_vec = glm::cross( &a_vec, &original );
            a_vec = a_vec * dx;
            b_vec = b_vec * dy;

            let pos = self.transforms.get_position(self.target) + b_vec * 10.0 + a_vec * 10.0;
            self.transforms.set_position(self.target, pos);
        }

        if self.is_left_mouse_dragging == true { 
            let dx = (pos.x - self.last_mouse_input.x) * 200.0;
            let dy = (pos.y - self.last_mouse_input.y) * 200.0;
            self.last_mouse_input = pos;
            self.lon -= dx;

            if self.lon < 0.0 {
                self.lon += 360.0;
            }

            if self.lon > 360.0 {
                self.lon -= 360.0;
            }

            self.lat += dy;
            self.lat = self.lat.clamp(-85.0, 85.0);     
        }
        
        self.last_mouse_input.x = pos.x;
        self.last_mouse_input.y = pos.y;
 
    }

    fn handle_event( &mut self, event : &event::WindowEvent, app : &app::App ){

        if let event::WindowEvent::MouseInput { state, button, ..} = event {
            if matches!( state, event::ElementState::Pressed ) { 
                self.mouse_pressed( *button == MouseButton::Left, *button == MouseButton::Middle);
            } 
            else if matches!( state, event::ElementState::Released ) {
                self.mouse_pressed( false, false);
            }
        }
    
        if let event::WindowEvent::MouseWheel { delta, .. } = event {
    
            match delta {
                event::MouseScrollDelta::LineDelta( _x, y) => {
                    self.distance += y;
                }
                _ => ()
            }
            
        }
    
        if let event::WindowEvent::CursorMoved{ position, .. } = event {
            self.mouse_input(*position, [app.input_state.window_size.0 as f32, app.input_state.window_size.1 as f32]);
        }
    }

    fn update(&mut self) {

            self.distance = self.distance.clamp(0.01, 100.0);
            
            let lat_r = self.lat.to_radians();
            let lon_r  = self.lon.to_radians();
            let pos = glm::vec3( lat_r.cos() * lon_r.sin(), lat_r.sin(), lat_r.cos() * lon_r.cos() ) * self.distance;
            self.transforms.set_position(self.eye, pos);
    }
}


impl Camera for OrbitCamera{

    fn get_view_matrix(&self) -> glm::Mat4 {

        let eye_pos = self.transforms.get_world_position(self.eye);
        let target_pos = self.transforms.get_world_position( self.target );

        glm::look_at(&eye_pos, &target_pos, &glm::vec3(0.0, 1.0, 0.0))
    }

    fn get_perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective( self.aspect_ratio, self.fov, self.near, self.far)
    }
}


struct FrameData {
    shader : glh::GlslProg,
    vao : glh::Vao,
    camera : OrbitCamera,
}

fn m_setup(app: &mut app::App) -> FrameData {

    let gl = &app.gl;

    let geo =  glh::geo::Geometry::axis(2.0); //glh::geo::Geometry::circle(0.0, 0.0, 1.0, false);
    let shader = glh::StockShader::new().color().build(gl);
    let vao = glh::Vao::new_from_attrib(gl, &geo.attribs, &shader).unwrap();

    let aspect_ratio = app.input_state.window_size.0 as f32 / app.input_state.window_size.1 as f32;

    let camera = OrbitCamera::new( aspect_ratio, 45.0, 0.0001, 1000.0 );
    
    FrameData {
        vao,
        shader,
        camera,
    }
}

fn m_event( _app : &mut app::App, _data : &mut FrameData, event : &event::WindowEvent ){

    _data.camera.handle_event(event, _app);
    // if let event::WindowEvent::KeyboardInput { .. } = event {
    // }
}

fn m_update(
    app: &mut app::App,
    data: &mut FrameData,
    _ui: &egui::CtxRef,
) {
    let gl = &app.gl;
    let circle_shader = &data.shader;
    let circle_vao = &data.vao;

    let camera = &mut data.camera;


    egui::SidePanel::new( egui::panel::Side::Left, "camera settings").show(_ui, |ui| {

        ui.columns(3, |ui_label| {
            ui_label[0].label("camera lat lon");
            ui_label[1].add( egui::widgets::DragValue::new( &mut camera.lat ) );
            ui_label[2].add( egui::widgets::DragValue::new( &mut camera.lon ) );
        });



        ui.columns(4, |ui_label| {
            let target_transform = camera.transforms.get_transform_mut( camera.target );
            ui_label[0].label("target_position");
            ui_label[1].add( egui::widgets::DragValue::new( &mut target_transform.position.x ).speed(0.01));
            ui_label[2].add( egui::widgets::DragValue::new( &mut target_transform.position.y ).speed(0.01));
            ui_label[3].add( egui::widgets::DragValue::new( &mut target_transform.position.z ).speed(0.01));
        });
    });


    camera.aspect_ratio =  app.input_state.window_size.0 as f32 / app.input_state.window_size.1 as f32;
    camera.update();
    //let target_id = camera.target;
    //camera.transforms.set_rotation( &target_id, glm::vec3( 0.0, app.frame_number as f32 * 0.001,  0.0) );

    let persp_matrix = camera.get_perspective_matrix();
    let view_matrix = camera.get_view_matrix();


    glh::clear(gl,0.8, 0.8, 0.8, 1.0);

    circle_shader.bind(gl);

    unsafe{ 
        gl.cull_face(glow::FRONT_AND_BACK);
    }

    circle_shader.set_uniform_mat4(gl, glh::StockShader::uniform_name_perspective_matrix(), &persp_matrix);
    circle_shader.set_view_matrix(gl, &view_matrix);

    let mut model_view = glm::Mat4::identity();
    model_view = glm::translate(&model_view, &glm::vec3(0.0, 0.0, 0.0 ));
    model_view = glm::scale(&model_view, &glm::vec3(1.0, 1.0, 1.0));
    
    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &model_view );
    circle_shader.set_uniform_4f( gl, glh::StockShader::uniform_name_color(), &[1.0, 1.0, 1.0, 1.0] );

    circle_vao.draw(gl, glow::LINES);
    
    let target_t = camera.transforms.get_world_matrix(camera.target);
    circle_shader.set_uniform_mat4( gl, glh::StockShader::uniform_name_model_matrix(), &target_t );
    circle_vao.draw(gl, glow::LINES);
    
    circle_shader.unbind(gl);
}

fn main() {
    app::AppBuilder::new(
        app::AppSettings {
            window_size: (1920, 1080),
            window_title: "simple app",
        },
        m_setup,
    )
    .event(m_event)
    .run(m_update);
}
