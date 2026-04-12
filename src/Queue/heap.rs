struct MaxHeap { 
    data:Vec<i32>
}

impl MaxHeap{
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn push(&mut self,val : i32)  {
        self.data.push(val);

        // hipyfy the heap ; 

        let mut i =  self.data.len() -1 ;

        while i > 0  {
            let parent_index = i/2;

            if self.data[parent_index] <= self.data[i] {
                self.data[i] = self.data[parent_index];
                self.data[parent_index] = val;
                i = i/2;
            }else {
                break;
            }
        }
    }


    fn print_elements(&mut self) {
         for i in 0..self.data.len() {
            println!("{:?}",self.data[i]);
         }
    }


    fn delete_element(&mut self) -> Option<i32> {
        if self.data.is_empty() { return  None};

        let response = self.data[0];

        let mut i =0 ;
         
         while i < self.data.len() {
            let mut c1 = i*2 +1;
            let mut c2 = i*2 +2;

            if self.data[c1]  >= self.data[c2] {
                self.data[i] = self.data[c1];
                i = c1 ;
            }else {
                self.data[i] = self.data[c2];
                i = c2;
            }
         }

        Some(response)
    }
}

pub fn heap_operation() {

    println!("Bellow are the heap operations ");

    let mut  heap_a = MaxHeap::new();
    
    for i in 0..8 { 
        heap_a.push(i);
    };

    heap_a.print_elements();

}