use std::{cell::RefCell, collections::{VecDeque, linked_list}, fmt::Debug, rc::Rc};


type Link<T> = Option<Rc<RefCell<T>>>;

#[derive(Debug)]
struct TreeNode<T>{
    val : T,
    left : Link<T>,
    right : Link<T>
}

#[derive(Debug)]
struct Tree<T>{
    head : Link<T>
}
// commeent 


impl <T: Debug> Tree<T>{
    fn new(&mut self,val : T)->Self {
        Tree { head: None }
    }


}


fn queue_operations(){
    // let mut q: VecDeque<Link<u32>> = VecDeque::new();



    
}