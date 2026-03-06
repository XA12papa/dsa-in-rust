use std::clone;
use std::fmt::Debug;
use std::mem::take;
use std::rc::Rc;
use std::cell::RefCell;

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

    println!();
}