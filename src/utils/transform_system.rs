use std::{collections::HashMap, ops::{Div}};

use glam;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
   pub position: glam::Vec3,
   pub rotation: glam::Vec3,
   pub scale: glam::Vec3,

   pub model_matrix : glam::Mat4,
}

impl Transform {
    fn new() -> Self {
        Transform {
            position: glam::vec3(0.0, 0.0, 0.0),
            rotation: glam::vec3(0.0, 0.0, 0.0),
            scale: glam::vec3(1.0, 1.0, 1.0),
            model_matrix : glam::Mat4::IDENTITY,
        }
    }
}

pub type NodeId = u64;

#[derive(Debug)]
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
    
    pub fn get_transform_mut(&mut self, key : NodeId ) -> &mut Transform {
        &mut self.transforms.get_mut(&key).unwrap().transform
    }
    
    pub fn get_transform(&self, key : NodeId ) -> &Transform {
        &self.transforms.get(&key).unwrap().transform
    }

    pub fn get_position(&self, id : NodeId)  -> glam::Vec3{
        self.transforms.get(&id).unwrap().transform.position
    }

    pub fn get_rotation(&self, id : NodeId)  -> glam::Vec3{
        self.transforms.get(&id).unwrap().transform.rotation
    }

    pub fn get_scale(&self, id : NodeId)  -> glam::Vec3{
        self.transforms.get(&id).unwrap().transform.scale
    }

    pub fn set_position(&mut self, id : NodeId, v: glam::Vec3) {
        self.transforms.get_mut(&id).unwrap().transform.position = v;
    }

    pub fn set_rotation(&mut self, id : NodeId, v: glam::Vec3) {
        self.transforms.get_mut(&id).unwrap().transform.rotation = v;
    }

    pub fn set_scale(&mut self, id : NodeId, v: glam::Vec3) {
        self.transforms.get_mut(&id).unwrap().transform.scale = v;
    }

    pub fn has_parent(&self, id : NodeId)  -> bool {
        self.transforms.get(&id).unwrap().parent.is_some()
    }

    pub fn set_parent(&mut self, id : NodeId, parent : NodeId, keep_current_transform : bool ){

        let (w_pos, w_rot, w_scale ) = {
            ( self.get_world_position(id), self.get_world_rotation(id), self.get_world_scale(id) )
        };
        

        let parent_node = self.transforms.get_mut(&parent).unwrap();
        parent_node.children.push(id);
    
        let node = self.transforms.get_mut(&id).unwrap();
        node.parent = Some(parent);

        if keep_current_transform {
            self.set_world_position(id,&w_pos);
            self.set_world_rotation(id,&w_rot);
            self.set_world_scale(id, &w_scale);
        }

    }

    pub fn clear_parent(&mut self, id : NodeId, keep_current_transform : bool ){

        let (w_pos, w_rot, w_scale ) = {
            ( self.get_world_position(id), self.get_world_rotation(id), self.get_world_scale(id) )
        };
          
        let parent_id =  {
            let node = self.transforms.get_mut(&id).unwrap();
            let parent_id = node.parent;
            node.parent = None;
            parent_id
        };

        // remove child from parent
        if let Some(parent_id) = parent_id {
            let parent_node = self.transforms.get_mut(&parent_id).unwrap();
            parent_node.children.retain( |child| {
                *child != id
            });
        }

        if keep_current_transform {
            self.set_world_position(id,&w_pos);
            self.set_world_rotation(id,&w_rot);
            self.set_world_scale(id, &w_scale);
        }
    }

    pub fn descend_children<F: Fn(&mut TransformNode, Option<&mut TransformNode>)>(&mut self, id : NodeId, f : F){

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



    pub fn set_world_position(&mut self, id : NodeId, world_pos : &glam::Vec3) {

        let parent_id_opt ={
            self.transforms.get_mut(&id).unwrap().parent
        };
        
        if let Some(parent_id) = parent_id_opt {
           let world_pos = {
                self.get_world_matrix(parent_id).inverse() * glam::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0)
           };
            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.position = glam::vec3(world_pos.x, world_pos.y, world_pos.z);
        }else{
            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.position = *world_pos;
        }
    }

    pub fn set_world_scale(&mut self, id : NodeId, world_scale : &glam::Vec3){
        
        let parent_id_opt ={
            self.transforms.get_mut(&id).unwrap().parent
        };

        if let Some(parent_id) = parent_id_opt {

            let inv_scale = {
                    //world_scale.component_div( &self.get_world_scale(parent_id) )
                    world_scale.div( self.get_world_scale(parent_id) )
            };
            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.scale = inv_scale;

        }else{
            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.scale = *world_scale;
        }
    }

    pub fn set_world_rotation(&mut self, id : NodeId, world_rot : &glam::Vec3){
        
        let parent_id_opt ={
            self.transforms.get_mut(&id).unwrap().parent
        };

        if let Some(parent_id) = parent_id_opt {
               let parent_rot = {
                    self.get_world_rotation(parent_id)
           };

            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.rotation = *world_rot - parent_rot;

        }else{
            let mut node = self.transforms.get_mut(&id).unwrap();
            node.transform.rotation = *world_rot;
        }
    }

    pub fn get_world_position(&self, id : NodeId) -> glam::Vec3 {
        let world_matrix = self.get_world_matrix(id);
        let world_position = world_matrix * glam::vec4(0.0,0.0,0.0, 1.0); // glam::vec4(0.0, 0.0, 0.0, 1.0);
        glam::vec3(world_position.x, world_position.y, world_position.z)
    }

    pub fn get_world_scale(&self, id : NodeId) -> glam::Vec3 {
        let mut node= self.transforms.get(&id).unwrap();
        let mut scale = self.get_scale(id);

        while node.parent.is_some() {
            if let Some(parent) = node.parent { 
                let parent_scale : glam::Vec3 = self.get_scale(parent);
                scale = scale * parent_scale;
                node = self.transforms.get(&parent).unwrap();
            }
        }

        scale
    }

    pub fn get_world_rotation(&self, id : NodeId) -> glam::Vec3 {
        let mut node= self.transforms.get(&id).unwrap();
        let mut rotation =  self.get_rotation(id);

        while node.parent.is_some() {

            if let Some(parent) = node.parent { 
                let parent_rotation : glam::Vec3 = self.get_rotation(parent);
                rotation = parent_rotation + rotation;
                node = self.transforms.get(&parent).unwrap();
            }

        }

        rotation
    }

    pub fn get_world_matrix(&self, id : NodeId) -> glam::Mat4 {
        
        let mut node= self.transforms.get(&id).unwrap();
        let mut matrix = self.get_model_matrix(id);

        while node.parent.is_some() {

            if let Some(parent) = node.parent { 
                let model_matrix = self.get_model_matrix(parent);
                matrix = model_matrix * matrix;
                node = self.transforms.get(&parent).unwrap();
            }   

        }

        matrix
    }

    pub fn get_model_matrix(&self, id : NodeId) -> glam::Mat4 {
        let transform = self.transforms.get(&id).unwrap().transform;
        let (a, b, c) = (transform.rotation.x, transform.rotation.y, transform.rotation.z);
        let model_matrix  = glam::Affine3A::from_scale_rotation_translation(transform.scale, glam::Quat::from_euler(glam::EulerRot::XYZ, a, b, c) , transform.position);
        glam::Mat4::from(model_matrix)
    }
}