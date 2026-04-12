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


impl <T: Debug + Copy +PartialEq> Tree<T> {
    fn new() -> Self {
        return Tree { root: None, node_count: 0 };
    }

    fn build_tree(&mut self,arr:&[T],i : usize) -> Link<T>{
        if  i >= arr.len() {
            return None;
        }
        self.node_count += 1 ;
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

    fn print_tree(&mut self ){
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
                    // println!("None")
                }
            }

        }
    }


    fn search(&mut self,val : T) -> Option<T>{
        if self.root.is_none()  {return  None;}

        let mut s : VecDeque<Link<T>> =  VecDeque::new();

        s.push_back(self.root.clone());

        while !s.is_empty() {
            let current = s.pop_front().unwrap();
            match current {
                Some(node) =>{
                    if node.borrow_mut().val  == val { 
                            return Some(val);
                    }
                    if  node.borrow().left.is_some() { s.push_back(node.borrow_mut().left.clone()); }
                    if  node.borrow().right.is_some() {s.push_back(node.borrow_mut().right.clone());}
                }None =>{}
            }
        }
        
        None
    }
}




enum Transversal {
    PreOrder,
    InOrder,
    PostOrder
}

fn  tranverse<T: Debug + Copy +PartialEq> (root : Link<T>, order : Transversal) {
    if root.is_some() { 
        let node = root.unwrap();
        match  order {
            Transversal::PreOrder => {
                println!("{:?}",node.borrow().val);
                tranverse(node.borrow_mut().left.clone(), Transversal::PreOrder);
                tranverse(node.borrow_mut().right.clone(), Transversal::PreOrder);
            },
            Transversal::InOrder => {
                tranverse(node.borrow_mut().left.clone(), Transversal::InOrder);
                println!("{:?}",node.borrow().val);
                tranverse(node.borrow_mut().right.clone(), Transversal::InOrder);
            },
            Transversal::PostOrder => {
                tranverse(node.borrow_mut().left.clone(), Transversal::PostOrder);
                tranverse(node.borrow_mut().right.clone(), Transversal::PostOrder);
                println!("{:?}",node.borrow().val);
            }
        }
    }
}






pub fn tree_operations() {

    println!("Below are tree operations ");


    let mut tree_A = Tree::new();
    let mut arr =[20,30,40,55,12,76,-1,10];

    tree_A.initiate(&arr);

    tree_A.print_tree();

    let result = tree_A.search(-1);

    match  result {
        Some(iteme  )=>{println!("The item  ( {:?} ) exists ", iteme)},
        None =>{println!("The item you are searching doesnt exists")}
    };


    println!("Pre Order Transversal ");
    tranverse(tree_A.root.clone(), Transversal::PreOrder);
    println!("In Order Transversal ");
    tranverse(tree_A.root.clone(), Transversal::InOrder);
    println!("Post Order Transversal ");
    tranverse(tree_A.root.clone(), Transversal::PostOrder);




}