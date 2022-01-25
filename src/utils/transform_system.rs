use std::{collections::HashMap};

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

    pub fn has_parent(&mut self, id : &NodeId)  -> bool {
        self.transforms.get(id).unwrap().parent.is_some()
    }

    pub fn set_parent(&mut self, id : &NodeId, parent : NodeId, keep_current_transform : bool ){

        let (w_pos, w_rot, w_scale ) = {
            ( self.get_world_position(id), self.get_world_rotation(id), self.get_world_scale(id) )
        };
        

        { // add the children to the parent node
            let parent_node = self.transforms.get_mut(&parent).unwrap();
            parent_node.children.push(*id);
        }

        {
            let node = self.transforms.get_mut(id).unwrap();
            node.parent = Some(parent);
        }

    
        if keep_current_transform {
            self.set_world_position(id,&w_pos);
            self.set_world_rotation(id,&w_rot);
            self.set_world_scale(id, &w_scale);
        }
    }

    pub fn clear_parent(&mut self, id : &NodeId, keep_current_transform : bool ){

        if keep_current_transform {
            let pos =  self.get_position(id);
            self.set_world_position(id, &pos);

            let scale =  self.get_scale(id);
            self.set_world_scale(id, &scale );

            let rot = self.get_rotation(id);
            self.set_world_rotation(id, &rot);

            println!("set world transform!");
        }
          
        let parent_id =  {
            let node = self.transforms.get_mut(id).unwrap();
            let parent_id = node.parent;
            node.parent = None;
            parent_id
        };

        // check if there's a parent
        if let Some(parent_id) = parent_id {
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



    pub fn set_world_position(&mut self, id : &NodeId, world_pos : &glm::Vec3) {

        let parent_id_opt ={
            self.transforms.get_mut(id).unwrap().parent
        };

        if let Some(parent_id) = parent_id_opt {
           let world_pos = {
               self.get_world_matrix(&parent_id) * glm::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0)
           };
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.position = glm::vec3(world_pos.x, world_pos.y, world_pos.z);
        }else{
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.position = *world_pos;
        }
    }

    pub fn set_world_scale(&mut self, id : &NodeId, world_scale : &glm::Vec3){
        
        let parent_id_opt ={
            self.transforms.get_mut(id).unwrap().parent
        };

        if let Some(parent_id) = parent_id_opt {
               let inv_scale = {
                    glm::vec3(1.0, 1.0, 1.0).component_mul( &self.get_world_scale(&parent_id) )
           };
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.scale = node.transform.scale.component_mul(&inv_scale);
        }else{
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.scale = *world_scale;
        }
    }

    pub fn set_world_rotation(&mut self, id : &NodeId, world_rot : &glm::Vec3){
        
        let parent_id_opt ={
            self.transforms.get_mut(id).unwrap().parent
        };

        if let Some(parent_id) = parent_id_opt {
               let parent_rot = {
                    self.get_world_rotation(&parent_id)
           };
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.rotation = node.transform.rotation + parent_rot;
        }else{
            let mut node = self.transforms.get_mut(id).unwrap();
            node.transform.rotation = *world_rot;
        }
    }

    pub fn get_world_position(&self, id : &NodeId) -> glm::Vec3 {
        let world_matrix = self.get_world_matrix(id);
        let world_position = world_matrix * glm::vec4(0.0,0.0,0.0, 1.0); // glm::vec4(0.0, 0.0, 0.0, 1.0);
        glm::vec4_to_vec3(&world_position)
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