use std::{collections::HashMap, hash::Hash};

use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
   pub position: glm::Vec3,
   pub rotation: glm::Vec3,
   pub scale: glm::Vec3,

   pub model_matrix : glm::Mat4,
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

pub type NodeId = u64;

pub struct TransformNode {
    parent : Option<NodeId>,
    children : Vec<NodeId>,
    transform : Transform,
}


#[derive(Default)]
pub struct TransformSystem {
    transforms : HashMap<NodeId, TransformNode>,
    current_id : NodeId,
}

impl TransformSystem {
    pub fn new() -> Self{ 
        TransformSystem::default()
    }

    pub fn keys<'a>(&'a self) -> std::collections::hash_map::Keys<'a, NodeId, TransformNode>  {
        self.transforms.keys()
    }

    pub fn new_transform<'a>(&'a mut self) -> (NodeId, Option<&mut TransformNode>) {
        

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
    

    pub fn get_position(&self, id : &NodeId)  -> glm::Vec3{
        self.transforms.get(id).unwrap().transform.position
    }

    pub fn get_rotation(&self, id : &NodeId)  -> glm::Vec3{
        self.transforms.get(id).unwrap().transform.rotation
    }

    pub fn get_scale(&self, id : &NodeId)  -> glm::Vec3{
        self.transforms.get(id).unwrap().transform.scale
    }

    pub fn set_position(&mut self, id : &NodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.position = v;
    }

    pub fn set_rotation(&mut self, id : &NodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.rotation = v;
    }

    pub fn set_scale(&mut self, id : &NodeId, v: glm::Vec3) {
        self.transforms.get_mut(id).unwrap().transform.scale = v;
    }



    pub fn set_parent(&mut self, id : &NodeId, parent : Option<NodeId>){

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

    pub fn descend_children<F: Fn(&mut TransformNode, Option<&mut TransformNode>)>(&mut self, id : &NodeId, f : F){

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

    pub fn get_world_scale(&self, id : &NodeId) -> glm::Vec3 {
        let mut node= self.transforms.get(id).unwrap();
        let mut scale = self.get_scale(id);

        while node.parent.is_some() {

            if let Some(parent) = node.parent { 
                let parent_scale : glm::Vec3 = self.get_scale(&parent);
                scale = scale.component_mul(&parent_scale);
                node = self.transforms.get(&parent).unwrap();
            }   

        }

        scale
    }

    pub fn get_world_position(&self, id : &NodeId) -> glm::Vec3 {

        let position = self.get_position(id);        
        let world_matrix = self.get_world_matrix(id);
        let world_position = world_matrix * glm::vec4(0.0,0.0,0.0, 1.0); // glm::vec4(0.0, 0.0, 0.0, 1.0);
        
        glm::vec4_to_vec3(&world_position)
    }

    pub fn get_world_rotation(&self, id : &NodeId) -> glm::Vec3 {
        let mut node= self.transforms.get(id).unwrap();
        let mut rotation = self.get_rotation(id);

        while node.parent.is_some() {

            if let Some(parent) = node.parent { 
                let parent_rotation : glm::Vec3 = self.get_rotation(&parent);
                rotation = parent_rotation + rotation;
                node = self.transforms.get(&parent).unwrap();
            }

        }


        rotation
    }

    pub fn get_world_matrix(&self, id : &NodeId) -> glm::Mat4 {
        
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

    pub fn get_model_matrix(&self, id : &NodeId) -> glm::Mat4 {
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