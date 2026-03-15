use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

struct Node{
    val : i32 ,
    next : Link,
    prev :Link
}


struct FrontMiddleBackQueue {
    head : Link,
    tail : Link,
    count: u32,
}


impl FrontMiddleBackQueue{
    fn new() -> Self{
        return FrontMiddleBackQueue { head: None, tail : None,count: 0 };
    }

    fn push_front(&mut self, val :i32){
        let mut new_node =  Rc::new(RefCell::new(
            Node{
                val,
                next:None,
                prev:None
            }
        ));

        match self.head.take() { 
            Some(node) =>{
                new_node.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(new_node.clone());

                self.head = Some(new_node);

            }None =>{
                self.head =Some(new_node.clone());
                self.tail= Some(new_node);
            }
        }

        self.count +=1 ;
    }


    fn push_middle(&mut self, val: i32) {
        if self.count == 0 || self.count == 1 { 
            self.push_front(val);
            return;
        }
        let mut i = 0  ;

        let mut  current = self.head.clone();

        while current.is_some() && i < self.count/2 {
                current = current.unwrap().borrow_mut().next.clone();

                i +=1 ;
        }

        match current {
            Some(node) =>{
                let mut new_node =  Rc::new(RefCell::new(
                    Node{
                        val,
                        next:None,
                        prev:None
                    }
                ));

                let mut prev = node.borrow_mut().prev.clone();
                if let Some(prev_node ) = prev { 
                    prev_node.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(prev_node);
                };

                new_node.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(new_node);


                self.count += 1 ;
                
            }None => { }
        }
    }


    fn push_back(&mut self ,val :i32) {
        let mut new_node =  Rc::new(RefCell::new(
            Node{
                val,
                next:None,
                prev:None
            }
        ));


        match  self.tail.take() {
            Some(node) =>{
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(node);

                self.tail = Some(new_node);
            }None =>{
                self.push_front(val);
            }
        }
    }


    fn pop_front(&mut self) -> i32 {
        match  self.head.take(){
            Some(node) =>{
                self.head = node.borrow_mut().next.clone();
                self.count -= 1 ;
                return  node.borrow().val;
            }None =>{ return  -1}
        }

 
    }

    fn pop_back(&mut self) -> i32 {
        match self.tail.take(){
            Some(node) =>{
                self.tail = node.borrow_mut().prev.clone();
                self.count -=1 ;

                return  node.borrow().val;
            }None =>{ -1}
        }
    }




    pub fn pop_middle(&mut self) ->i32{

        if self.count == 0 {
            return -1;
        }

        let mid_index = (self.count - 1) / 2;

        let mut current = self.head.clone();

        for _ in 0..mid_index {
            if let Some(node) = current.clone() {
                current = node.borrow().next.clone();
            }
        }

        let mid_node = current.unwrap();
        let value = mid_node.borrow().val;

        let prev = mid_node.borrow_mut().prev.take();
        let next = mid_node.borrow_mut().next.take();

        match prev.clone() {
            Some(p) => {
                p.borrow_mut().next = next.clone();
            }
            None => {
                self.head = next.clone();
            }
        }

        match next.clone() {
            Some(n) => {
                n.borrow_mut().prev = prev.clone();
            }
            None => {
                self.tail = prev.clone();
            }
        }

        self.count -= 1;

        value
    }
}

