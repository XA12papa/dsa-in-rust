use core::panic;
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};



type Link<T>  = Option<Rc<RefCell<Node<T>>>>;


#[derive(Debug)]
struct Node<T>{
    val : T,
    left : Link<T>,
    right : Link<T>
}


#[derive(Debug)]
struct Tree<T>{
    root: Link<T>,
    node_count : u32,
}


impl <T: Debug + Copy> Tree<T> {
    fn new() -> Self {
        return Tree { root: None, node_count: 0 };
    }

    fn build_tree(&mut self,arr:&[T],i : usize) -> Link<T>{
        if  i >= arr.len() {
            return None;
        }

        return Some(Rc::new(RefCell::new(
            Node { val: arr[i],
                    left : self.build_tree(arr, i*2 +1),
                    right: self.build_tree(arr, i*2 +2 ) 
                }
        )));
    }
    fn initiate(&mut self,arr  : &[T]) {
        let root = self.build_tree(arr, 0);
        self.root = root;
    }

    fn print_Tree(&mut self ){
        if self.root.is_none() {
            println!("The tree is Empty !! ")
        };

        let mut queue : VecDeque<Link<T>> =  VecDeque::new();

        queue.push_back(self.root.clone());

        while !queue.is_empty() {
            let current  = queue.pop_front().unwrap();

            match  current {
                Some(node ) =>{
                    queue.push_back(node.borrow_mut().left.clone());
                    queue.push_back(node.borrow_mut().right.clone());
                    println!("{:?}",node.borrow().val);

                },
                None =>{
                    println!("None")
                }
            }

        }
    }
}


pub fn Tree_operations() {

    println!("Below are tree operations ");


    let mut tree_A = Tree::new();

    let mut arr =[20,30,40,55,12,76,-1,10];

    tree_A.initiate(&arr);

    tree_A.print_Tree();


}