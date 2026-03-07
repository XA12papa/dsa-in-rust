use std::clone;
use std::fmt::Debug;
use std::mem::take;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::sync::Arc;

type Link<T>  = Option<Rc<RefCell<Node<T>>>> ;



#[derive(Debug)]
struct Node<T>{
    value :T ,  
    next : Link<T>,
    prev : Link<T>
}
#[derive(Debug)]
struct linked_list<T>{
    head : Link<T>,
    tail : Link<T>
}


impl <T :Debug> linked_list<T>{

    fn new() -> Self{
        linked_list {head : None, tail : None}
    }

    fn push_front(&mut self,val : T) {
        let new_node = Rc::new(
            RefCell::new(
                Node {
                    value :val,
                    next :None,
                    prev :None
                }
            )
        ); // can have multiple owners 

        match  self.head.take(){
            Some(old_head) =>{
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());

                self.head = Some(new_node);
            },
            None =>{
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    fn insert_at(&mut self, index: usize, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value:val,
            next: None,
            prev: None,
        }));

        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current.clone() {
            if i == index {
                let prev = node.borrow().prev.clone();

                new_node.borrow_mut().next = Some(node.clone());
                new_node.borrow_mut().prev = prev.clone();

                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().next = Some(new_node.clone());
                }

                node.borrow_mut().prev = Some(new_node.clone());

                if index == 0 {
                    self.head = Some(new_node);
                }

                break;
            }

            current = node.borrow().next.clone();
            i += 1;
        }
    }

    fn delete_at(&mut self, index : u32){
        let mut  i =  0 ;
        let mut  current  = self.head.clone();

        while let Some(node) = current.clone() {
            if  i == index {
                if index ==  0 {
                    let new_head = node.borrow().next.clone();
                    match new_head {
                        Some(item) =>{
                            item.borrow_mut().prev = None ; 
                            self.head = Some(item);
                        },
                        None =>{
                            self.head = None;
                        }
                    }

                    break;
                }

                let prev = node.borrow().prev.clone().unwrap();
                let next = node.borrow().next.clone();

                match next {
                    Some(item) =>{
                        prev.borrow_mut().next = Some(item.clone());
                        item.borrow_mut().prev = Some(prev);
                    },
                    None =>{
                        prev.borrow_mut().next = None
                    }
                }
            }   

            i += 1;
            current = node.borrow().next.clone();

        }
    }
    fn push_back(&mut self, val : T){
        let new_node  = Rc::new(
            RefCell::new(
                Node{
                    value : val,
                    next :None,
                    prev : None
                }
            )
        );


        match self.tail.take(){
            Some(old_tail) =>{
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);

                self.tail = Some(new_node);
            },
            None =>{
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

    }


    fn print_list(&mut self){
        let mut current = self.head.clone();
        

        println!();
        while let Some(node) = current {
            println!("{:?}",node.borrow().value);
            current =node.borrow().next.clone();
        }
        println!()
    }
}
pub fn list_operations(){
    println!();
    println!("Doubly linked_list code -> ") ; 

    let mut  list_A = linked_list::new();

    list_A.push_back(11);
    list_A.push_back(22);
    list_A.push_back(33);
    list_A.push_back(44);
    list_A.push_back(55);

    list_A.print_list();

    list_A.push_front(98);

    list_A.print_list();

    list_A.insert_at(2, 65  );

    list_A.print_list();


    list_A.delete_at(7  );

    list_A.print_list();

    println!();
}