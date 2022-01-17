extern crate piralib;
use std::collections::HashMap;

use nalgebra_glm as glm;
use piralib::gl_helper as glh;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Transform {
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,

    model_matrix : glm::Mat4,
}

impl Transform {
    fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            model_matrix : glm::Mat4::identity(),
        }
    }
}

type TransformNodeId = u64;

struct TransformNode {
    parent : Option<TransformNodeId>,
    children : Vec<TransformNodeId>,
    transform : Transform,
}


#[derive(Default)]
struct TransformSystem {
    transforms : HashMap<TransformNodeId, TransformNode>,
    current_id : TransformNodeId,
}

impl TransformSystem {
    pub fn new() -> Self{ 
        TransformSystem::default()
    }

    pub fn new_transform<'a>(&'a mut self) -> (TransformNodeId, Option<&mut TransformNode>) {
        

        self.current_id = self.current_id + 1;
        let transform = Transform::new();
        
        self.transforms.insert(self.current_id, TransformNode{
                parent : None,
                children : Vec::new(),
                transform
            }
        );
        

        (self.current_id, self.transforms.get_mut(&self.current_id))
    }
    

    fn get_position(&mut self, id : &TransformNodeId)  -> glm::Vec3{
        self.transforms.get_mut(id).unwrap().transform.position
    }

    fn get_rotation(&mut self, id : &TransformNodeId)  -> glm::Vec3{
        self.transforms.get_mut(id).unwrap().transform.rotation
    }

    fn get_scale(&mut self, id : &TransformNodeId)  -> glm::Vec3{
        self.transforms.get_mut(id).unwrap().transform.scale
    }

    fn set_position(&mut self, id : &TransformNodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.position = v;
    }

    fn set_rotation(&mut self, id : &TransformNodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.rotation = v;
    }

    fn set_scale(&mut self, id : &TransformNodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.scale = v;
    }



    fn set_parent(&mut self, id : &TransformNodeId, parent : Option<TransformNodeId>){

        if let Some(parent_id) = parent {

            {
                let parent_node = self.transforms.get_mut(&parent_id).unwrap();
                parent_node.children.push(*id);
            }

            {
                let node = self.transforms.get_mut(id).unwrap();
                node.parent = parent.clone();
            }

        }else if let Some(parent_id) = self.transforms.get(id).unwrap().parent{

            let parent_node = self.transforms.get_mut(&parent_id).unwrap();
            parent_node.children.retain( |child| {
                child != id
            });

        }
    }

    fn descend_children<F: Fn(&mut TransformNode, Option<&mut TransformNode>)>(&mut self, id : &TransformNodeId, f : F){

        let mut stack = Vec::new();
        stack.push(id.clone()); 
        while stack.is_empty() == false {
            let new_id = stack.pop().unwrap();
            
            let (node, parent) = unsafe {
                let a = self.transforms.get_mut(&new_id).unwrap() as *mut TransformNode;

                if let Some(parent_node) = (*a).parent {
                    let b = self.transforms.get_mut( &parent_node ).unwrap() as *mut _;
                    assert_ne!(a, b, "The two keys must not resolve to the same value");
                    (&mut *a, Some(&mut *b))    
                }else{
                    (&mut *a, None)
                }

            };

            f(node, parent);

            for child in &node.children.clone(){
                stack.push(child.clone());
            }
        }
    }


    fn get_world_matrix(&self, id : &TransformNodeId) -> glm::Mat4 {
        
        let mut node= self.transforms.get(id).unwrap();
        let mut matrix = self.get_model_matrix(id);

        while node.parent.is_some() {

            if let Some(parent) = node.parent { 
                let model_matrix = self.get_model_matrix(&parent);
                matrix = model_matrix * matrix;
                node = self.transforms.get(&parent).unwrap();
            }   

        }

        matrix
    }

    fn get_model_matrix(&self, id : &TransformNodeId) -> glm::Mat4 {
        let transform = self.transforms.get(id).unwrap().transform;
        let mut model_matrix = glm::Mat4::identity();

        model_matrix = glm::translate(&model_matrix, &transform.position);

        let mut rot_matrix = glm::Mat4::identity();
        rot_matrix = glm::rotate_x(&mut rot_matrix, transform.rotation.x);
        rot_matrix = glm::rotate_y(&mut rot_matrix, transform.rotation.y);
        rot_matrix = glm::rotate_z(&mut rot_matrix, transform.rotation.z);
        model_matrix = model_matrix * rot_matrix;

        model_matrix = glm::scale(&model_matrix, &transform.scale);

        model_matrix
    }
}

 struct FrameData {
    transforms_arena :  TransformSystem, //Arena<Transform>,
    node_a : TransformNodeId,
    node_b : TransformNodeId,
    node_c : TransformNodeId,

    shader : glh::GlslProg,
    vao : glh::Vao,
}

fn setup_fn(app : &mut piralib::app::App) -> FrameData {



    let mut ts = TransformSystem::new();
    let (aa, _) = ts.new_transform();
    let (bb, _) = ts.new_transform();
    let (cc, _) = ts.new_transform();

    ts.set_position(&aa,glm::vec3(1280.0, 720.0, 0.0));
    ts.set_rotation(&aa,glm::vec3(0.0, 0.0, 3.14 / 4.0));
    ts.set_scale(&aa, glm::vec3(1.0, 1.0, 1.0));


    ts.set_position(&bb,glm::vec3(250.0, 0.0, 0.0));
    ts.set_rotation(&bb,glm::vec3(0.0, 0.0, 0.0));
    ts.set_scale(&bb, glm::vec3(1.0, 1.0, 1.0));

    ts.set_position(&cc,glm::vec3(400.0, 0.0, 0.0));
    ts.set_rotation(&cc,glm::vec3(0.0, 0.0, 0.0));
    ts.set_scale(&cc, glm::vec3(1.0, 1.0, 1.0));

    
    ts.set_parent(&bb, Some(aa));
    ts.set_parent(&cc, Some(bb));


    println!( "{}", ts.get_world_matrix(&cc)) ;

    let geo_rect = piralib::gl_helper::geo::Geometry::rect(-100.0, -100.0, 200.0, 200.0);
    let shader = glh::stock_shader::StockShader::new().color().build(&app.gl);
    let vao = glh::Vao::new_from_attrib(&app.gl, &geo_rect.attribs, &shader).unwrap();


    FrameData {
        transforms_arena : ts,
        node_a : aa,
        node_b : bb,
        node_c : cc,
        vao,
        shader,
    }
}


fn update_fn(app : &mut piralib::app::App, data : &mut FrameData, _egui : &egui::CtxRef){


    let gl = &app.gl;

    let FrameData{vao, shader, transforms_arena, ..} = data;

    // glh::set_viewport(gl, x, y, width, height)
    glh::set_viewport(gl, 0, 0, app.input_state.window_size.0 as i32 * 2, app.input_state.window_size.1 as i32 * 2);
    glh::clear(gl, 0.3, 0.1, 0.13, 1.0);

    
    {
        transforms_arena.set_rotation(&data.node_a, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.01 ));
        let s = ((app.frame_number as f32) * 0.01).sin();
        transforms_arena.set_scale(&data.node_a, glm::vec3(s,s,s));
    }


    {
        transforms_arena.set_rotation(&data.node_b, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.01 ));
        let s = ((app.frame_number as f32) * 0.01 + 1.0).sin();
        transforms_arena.set_scale(&data.node_b, glm::vec3(s,s,s));
    }

    {
        transforms_arena.set_rotation(&data.node_c, glm::vec3(0.0, 0.0, app.frame_number as f32 * 0.01 ));
        let s = ((app.frame_number as f32) * 0.01 + 2.0).sin();
        transforms_arena.set_scale(&data.node_c, glm::vec3(s,s,s));
    }


    for id in transforms_arena.transforms.keys() {

        shader.bind(gl);
        shader.set_orthographic_matrix(gl, [app.input_state.window_size.0 as f32 *2.0, app.input_state.window_size.1 as f32 * 2.0]);
        shader.set_view_matrix(gl, &glm::Mat4::identity());

        let model_matrix =   transforms_arena.get_world_matrix(id);  // get_world_matrix(node_id, &transforms_arena);
        shader.set_model_matrix(gl, &model_matrix);


        let pos = model_matrix * glm::vec4(0.0, 0.0, 0.0, 1.0);
        let mouse_pos =  glm::vec3(app.input_state.mouse_pos.0 * 2.0, app.input_state.mouse_pos.1 * 2.0, 0.0);

        if glm::distance( &mouse_pos, &pos.xyz() ) < 100.0 {
            shader.set_color(gl, &[0.0, 0.0, 1.0, 1.0]);
        }else{
            shader.set_color(gl, &[1.0, 1.0, 0.0, 1.0]);
        }

        vao.draw(gl, glow::TRIANGLES);
        shader.unbind(gl);
    }
   
}


fn main() {
 
    piralib::app::AppBuilder::new( piralib::app::AppSettings{
     window_title : "transforms",
     window_size : (1280, 720),
    }, setup_fn).run(update_fn);

}

#[test]
fn scene_graph_test() {
    main();
}
