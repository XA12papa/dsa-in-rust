use std::fmt::Debug;




#[derive(Debug)]
struct Node<T>{
    val : T,
    prev : Option<Box<Node<T>>>
}


#[derive(Debug)]
struct Stack<T>{
    top:Option<Box<Node<T>>>,
    count : u32,
}



impl <T : Debug> Stack<T> {
    fn new() -> Self{
        return Stack {top: None,count : 0}
    }

    fn push(&mut self , val : T){
        let mut  new_node =  Box::new(
            Node {
                val : val,
                prev : None
            }
        );

        new_node.prev = self.top.take();
        self.top = Some(new_node);

        self.count += 1;
    }

    fn pop(&mut self) -> Option<Box<Node<T>>>{

        let top_element = self.top.take();

        match top_element{
            Some(mut node) =>{
                self.top = node.prev.take();
                self.count -= 1;
                return Some(node);
            },
            None =>{
                return None;
            }
        }
 
    }


    fn print_stack(&mut self) {
        let mut current =  &mut self.top;

        let mut index = self.count;
        println!();
        while let Some(node) = current {
            println!("{} -> {:?}",index,node.val);
            index -= 1;
            current  = &mut node.prev;
        }

        println!();
    }

}

pub fn stack_operations(){
    println!(" Below is all the stack operations  -> ");
    let mut stack_A = Stack::new();

    for i in 0..7 {
        stack_A.push(i*20);
    };

    stack_A.print_stack();

    for _ in 0..7 {
        // stack_A.push(i*20);
        stack_A.pop();
    };

    match stack_A.pop() {
        Some(node) =>{
            println!("The top value is  : {:?} ", node.val);
        }None =>{
            println!("The stack is empty")
        }
    }



}