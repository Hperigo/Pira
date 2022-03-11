use crate::event;
use glutin::dpi::PhysicalPosition;
use crate::utils::transform_system;
use nalgebra_glm as glm;
use crate::app;

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

    pub fn handle_event( &mut self, event : &event::WindowEvent, app : &app::App ){

        if let event::WindowEvent::MouseInput { state, button, ..} = event {
            if matches!( state, event::ElementState::Pressed ) { 
                self.mouse_pressed( *button == event::MouseButton::Left, *button == event::MouseButton::Middle);
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

    pub fn on_resize(&mut self, width : f32, height : f32) 
    {
        self.aspect_ratio = width / height;
    }

    pub fn update(&mut self) {

            self.distance = self.distance.clamp(0.01, 100.0);
            
            let lat_r = self.lat.to_radians();
            let lon_r  = self.lon.to_radians();
            let pos = glm::vec3( lat_r.cos() * lon_r.sin(), lat_r.sin(), lat_r.cos() * lon_r.cos() ) * self.distance;
            self.transforms.set_position(self.eye, pos);
    }


    pub fn get_target_world_matrix(&self) -> glm::Mat4 {
        self.transforms.get_world_matrix(self.target)
    }


    pub fn get_eye_position( &self ) -> glm::Vec3 {
        self.transforms.get_world_position( self.eye )
    }

    pub fn get_target_position( &self ) -> glm::Vec3 {
        self.transforms.get_world_position( self.target )
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
