use std::{cell::RefCell, fmt::Debug, rc::Rc};



type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T>{
    val : T,
    next : Link<T>
}


struct Queue<T>{
    head : Link<T>,
    tail : Link<T>,
}


impl <T:Debug + Copy> Queue<T>{
    fn new() -> Self { 
        return Queue { head: None, tail: None };
    }

    fn push(&mut self , val : T){
        let new_node = Rc::new(
            RefCell::new(
                Node{
                    val:val,
                    next:None
                }
            )
        );

        match self.tail.take(){
            Some(node) =>{
                node.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }None =>{
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }


    }

    fn pop(&mut self) -> Link<T>{
        let old_head = self.head.take();

        match old_head {
            Some(node) =>{
                self.head = node.borrow_mut().next.take();

                return Some(node);
            }None => None
        }
    }

    fn print_list(&mut self){


        let mut current = self.head.clone();

        println!("head ");
        while  let Some(node) = current {
            println!("{:?}",node.borrow().val);
            current = node.borrow().next.clone();
        }
        println!("")

    }

    fn peak(&mut self)->Link<T>{

        match  self.head.clone() {
            Some(node) =>{
                let new_node = Rc::new(
                    RefCell::new(
                        Node {
                            val : node.borrow_mut().val,
                            next : None
                        }
                    )
                );
                return Some(new_node);
            }
            None => None,
        }
    }

    fn size(&mut self)-> u64 { 
        let mut current  = self.head.clone();
        let mut count = 0 ;

        while let Some(node ) = current{
            count += 1;
            current = node.borrow_mut().next.clone();
        };

        return count;
    }
}







pub fn queue_operations(){
    println!();
    println!("The operations below are of queue");

    let mut queue_A = Queue::new();

    for i in 0..8 {
        queue_A.push(i);
    } ;

    queue_A.print_list();

    queue_A.pop();
    queue_A.print_list();

    println!("{:?}",queue_A.size())


}