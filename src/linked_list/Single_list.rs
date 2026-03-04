use std::{fmt::Debug, io::Cursor, ops::Index};



#[derive(Debug)]
struct node<T>{
    val : T,
    next : Option<Box<node<T>>>
}

#[derive(Debug)]
struct linked_list<T>{
    head : Option<Box<node<T>>>
}

impl <T:Debug> linked_list<T>{
    fn new() -> Self{
        linked_list { head: None }
    }

    fn push(&mut self , val :T){
        let new_node = Box::new(node{
            val,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }


    fn print_list(&self){

        let mut current = &self.head;
        println!();
        while let  Some(item) = current {
            println!( "{:?}" , item);
            current = &item.next;
        }   
        println!();
    }

    fn push_back(&mut self , val  : T) {
        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut  node.next; 
            // here we can access node.next because Rust automatically dereferences the reference.
            //  so node : &mut Box<Node<T>> ----> node : &mut Node<T>
        }
        // current stores the memory address of the Option variable that currently contains None.
        //So current is a mutable reference to the Option container, not to the value inside it.


        // Stack
        // ----------------
        //self.head : None    


        // None is just a value stored inside an Option variable.
        // current points to the Option variable, not to the None value.

        *current = Some(Box::new(node{
            val,
            next :None
        }))
    }

    fn insert_at(&mut self,val : T,index : u32 ) {
        let mut current  =&mut self.head;
        let mut i =  0 ;

        while  i<index {
            match current {
                Some(item) =>{
                    current = &mut item.next
                },
                None  =>{break;}
            }
            i += 1;
        }

        let new_node = Box::new( node{
            val,
            next : current.take()
        });

        *current = Some(new_node)
    }

}


pub fn list_operations(){
    println!();
    println!("below is code for singly linked_list ") ;
    println!();


    let mut list_a  = linked_list::new();

        list_a.push(10);
        list_a.push(20);
        list_a.push(30);
        list_a.push(40);

    list_a.print_list();
    


    let mut list_b = linked_list::new();


    for i in 0..5{
        list_b.push_back(i);
    }

    list_b.print_list();

    list_b.insert_at(78, 2);

    list_b.print_list();
}