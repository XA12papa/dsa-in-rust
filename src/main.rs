#![allow(dead_code)]

use Practice_Rust::linked_list::Double_list;
use Practice_Rust::linked_list::Single_list;
use Practice_Rust::Stack::Stack;
fn analyze_size(arr : &[i32]){
    println!("The first element is {:?}",arr[0]);
    println!("The size of the array is  {:?}",arr.len())
}

// structs 


#[derive(Debug)]
struct Status {
    x : f32,
    y : f32,
    name : String,
    health :  i32
}


// enums 

enum WebEvent{

    PageLoad,
    PageUnload,

    KeyPressed(char),
    Paste(String),

    Click {x : f32, y :f32},
}

struct Point {
    x : f64,
    y : f64
}

impl Point {
    fn origin() -> Point{
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x : f64 , y : f64) -> Point {
        Point { x, y }
    }

    fn translate(&mut self,x:f64 , y :f64){
         self.x += x;
         self.y += y;

    }

    fn display(&self){
        println!(" x : {} , y  : {} ",self.x,self.y);
    }
}
enum Arithmatic {
    Add , 
    Substract,
}

impl Arithmatic {
    fn run(&self,x : i32, y : i32) ->i32{
        match self {
            Self::Add => x + y  ,
            Self::Substract => x-y,
        }
    } 

}
fn handle_web_event(event : WebEvent){
    match  event {
        WebEvent::PageLoad => println!("The Page has been loaded"),
        WebEvent::PageUnload => println!("The Page is unloaded "),
        WebEvent::KeyPressed(c) => println!("The use pressed the '{}' key",c),
        WebEvent::Paste(str) => println!("The string used is {:?} ", str),
        WebEvent::Click { x, y } => println!("The y :{1} x :{0}",x,y)
    }
}

fn main() {

    // 
    let  xs: [i32;5] = [1,2,3,4,5];

    for (i,value)  in  xs.iter().enumerate() {
        println!("The {index} element is : {value}",index = i , value = value);
    }

    analyze_size(&xs[0..3]);


    // struct part 
    let rohan = Status {
        x : 3.5,
        y  :5.43,
        name  :String::from("Rohan"),
        health :234
    };


    println!("{:?}",rohan);

    let load  = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let key_pressed =  WebEvent::KeyPressed('b');
    let paste = WebEvent::Paste(String::from("superman"));
    let click = WebEvent::Click { x: 23.4, y: 4.33 };


    handle_web_event(load);
    handle_web_event(unload);
    handle_web_event(key_pressed);
    handle_web_event(paste);
    handle_web_event(click);


    let add = Arithmatic::Add;
    let response =  add.run(10, 15);

    println!("response : {}",response);


    // condinates of a person 

    let mut p1 = Point::new(2.55,6.55);
    p1.translate(4.65, 5.65);
    p1.display();

    Single_list::list_operations();
    Double_list::list_operations();
    Stack::stack_operations();
}
