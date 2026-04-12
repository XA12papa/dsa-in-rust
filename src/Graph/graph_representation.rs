use std::io;



fn print_matrix(matrix : &Vec<Vec<u32>>) {
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}


pub fn matrix_representation() {
    println!("The code below is matrix representation of graph ");

    println!("The No of nodes and edges in the graph  :");

    let mut input =  String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let num : Vec<u32> = input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("Please enter a valid number"))
    .collect();


    let width = num[0] as usize;
    let mut  matrix : Vec<Vec<u32>> = vec![vec![0 ;width]; width]; 

    print_matrix(&matrix);
    for _ in 0..num[1] {
        loop {
              let mut edge_input = String::new();

                println!("Enter the edge (source destination): ");

                io::stdin()
                .read_line(&mut edge_input)
                .expect("Please entre a valid number ");
                let edge: Vec<u32> = edge_input
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Please enter a valid number"))
                .collect();

                let source = edge[0] as usize;
                let destination = edge[1] as usize;

                if source < width && destination < width {
                    matrix[source][destination] = 1; 
                    matrix[destination][source] = 1; 

                    break;
                } else {
                    println!("Invalid edge: {} -> {}   try again ", source, destination);
                }
        }

    }

    print!("The adjacency matrix representation of the graph is :\n");
    print_matrix(&matrix);
}



pub fn list_representation() {
    println!("The code below is list representation  of graph");

    println!("The No of nodes and edges in the graph  :");

    let mut input =  String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num : Vec<u32> = input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("Please enter a valid number"))
    .collect();

    let width = num[0] as usize;


    let mut adjacency_list : Vec<Vec<u32>> = vec![Vec::new(); width];


    for _ in 0..num[1]{
        loop {
            let mut  edge_input  = String::new();
            println!("Enter the edge (source destination): ");
            io::stdin().read_line(&mut edge_input).expect("Failed to read line ");

            let edge: Vec<u32> = edge_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please enter a valid number"))
            .collect();

            let source = edge[0] as usize;
            let destination = edge[1] as usize;

            if source < width && destination < width {
                adjacency_list[source].push(destination as u32);
                adjacency_list[destination].push(source as u32);

                break;
            } else {
                println!("Invalid edge: {} -> {}   try again ", source, destination);
            }
        }
    }


    println!("The adjacency list representation of the graph is :");
    println!("{:?}", adjacency_list);
}

pub fn graph_representation() {
    // matrix_representation();
    list_representation();

}

