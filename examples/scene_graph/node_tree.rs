use std::{cell::RefCell, cell::RefMut, cell::Ref, collections::HashMap};
use std::{rc::Rc};
#[derive(Debug)]
struct Transform{
    position : f64,
}

impl Transform {
    fn new() -> Self{
        Transform { position : 0.0}
    }
}

type Container<T> = HashMap<usize, T>;
type NodeKey = usize;
#[derive(Debug)]
struct Tree<T> {
    container : Rc<RefCell<Container<T>>>,
    keys : Container<Node>,
}

struct DataView<'c, T> {
    key : NodeKey,
    context : Ref<'c, Container<T>>,
}

impl<'c, T> DataView<'c, T>{

    fn get(&'c self) -> &'c T {
        self.context.get(&self.key).unwrap()
    }
}

struct DataViewMut<'c, T> {
    key : NodeKey,
    context : RefMut<'c, Container<T>>,
}

impl<'c, T> DataViewMut<'c, T>{

    fn get<'a>(&'a mut self) -> &'a mut T {
        self.context.get_mut(&self.key).unwrap()
    }
}



impl<T> Tree<T> {

    pub fn new() -> Self{
        Tree{
          container : Rc::new(RefCell::new(Container::new())),
          keys : Container::new(),
        }
    }

    pub fn new_node(&mut self, data : T) -> NodeKey {
 
        // let mut data_container = self.container;
        let key = self.container.borrow().len();

        self.container.borrow_mut().insert( key, data );
        
        let node =Node{
            data_key : key,
            parent : None,
            children : vec![],
        };

        self.keys.insert(key, node.clone());
        
        key
    }

    fn get_root_nodes(&self) -> Vec<NodeKey> {
        
        let mut result  : Vec<NodeKey> = Vec::new();

        for node in &self.keys{
            if node.1.parent == None{
                result.push(*node.0);
            }
        }

        result
    }

    pub fn borrow_data<'a>(&'a self, key : usize) -> DataView<'a, T> {
        DataView{
            key : key,
            context : self.container.borrow(),
        }
    }

    pub fn borrow_data_mut<'c>(&'c mut self, key : usize) -> DataViewMut<'c, T>{
     DataViewMut{
            key : key,
            context : self.container.borrow_mut()
        }
    }


    pub fn set_parent(&mut self, child : NodeKey, parent : NodeKey ){
        self.keys.get_mut(&child).unwrap().parent = Some(parent);
        self.keys.get_mut(&parent).unwrap().children.push(child);
    }

    pub fn remove_parent(&mut self, child : NodeKey){
        
        let parent_key = self.keys.get(&child).unwrap().parent.unwrap();
        let children = &mut self.keys.get_mut(&parent_key).unwrap().children;
        
        let index = children.iter().position(|i| *i == child).unwrap();
        children.remove(index);

        self.keys.get_mut(&child).unwrap().parent = None;
    }

    fn descend_tree_helper<F>(&self, node : NodeKey, callback : &mut F, depth : usize) where F : FnMut( NodeKey, usize) {
        callback(node, depth );
        for child_key in &self.keys.get(&node).unwrap().children {
            self.descend_tree_helper( *child_key, callback , depth + 1 );
        }
    }

    fn descende_tree<F>(&self, key : NodeKey, callback : &mut F) where F : FnMut(NodeKey, usize){
        self.descend_tree_helper(key, callback, 0);
    }
    

    fn ascend_tree_helper<F>(&self, key : NodeKey, callback : &mut F, depth : usize) where F : FnMut(NodeKey, usize){
  
        callback(key, depth);

        let node = self.keys.get(&key).unwrap().parent;
        match node {
            Some(parent_key) => self.ascend_tree_helper(parent_key, callback, depth + 1),
            _ => ()
        };             
    }

    fn ascend_tree<F>(&self, key : NodeKey, callback : &mut F) where F : FnMut(NodeKey, usize){
        self.ascend_tree_helper(key, callback, 0);
    }
}

#[derive(Clone, Debug)]
struct Node {
    data_key : NodeKey,
    parent : Option<NodeKey>,
    children : Vec<NodeKey>,
}

impl Tree<Transform>{
    
    fn get_world_position(&self, key :NodeKey) -> f64 {
        
        let mut final_pos = 0.0;

        self.ascend_tree(key, &mut |key, depth|{ 
            final_pos = final_pos + self.container.borrow().get(&key).unwrap().position;
        });

        final_pos
    }
}



#[test]
fn name() {
   
    let mut ctx = Tree::new();
    let a = ctx.new_node(Transform::new() );
    let b = ctx.new_node(Transform::new() );
    let c = ctx.new_node(Transform::new() );
    
    ctx.set_parent(a, b);
    ctx.set_parent(b, c);
    
    ctx.borrow_data_mut(a).get().position = 1.0;
    ctx.borrow_data_mut(b).get().position = 10.0;
    ctx.borrow_data_mut(c).get().position = 100.0;

    println!("world position: {}", ctx.get_world_position(a));
    assert_eq!(ctx.get_world_position(a), 111.0);

    ctx.remove_parent(a);
    assert_eq!(ctx.get_world_position(a), 1.0);


    let data = &ctx.container;
     ctx.descende_tree(b, &mut | key, depth | {
            data.borrow_mut().get_mut(&key).unwrap().position = 100.0;
    });

    println!("world position: {}", ctx.get_world_position(b));
    assert_eq!(ctx.get_world_position(b), 200.0);
}