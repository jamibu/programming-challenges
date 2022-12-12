use std::collections::BinaryHeap;
use std::fs;

struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    row: usize,
    col: usize,
    elevation: i32,
    walkable: bool
}

fn main() {
    // let filename = "puzzleInput.txt";
    let filename = "example.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let grid: Vec<Vec<char>> = contents.strip_suffix("\n").unwrap().split("\n")
        .map(|l| l.chars().collect()).collect();
    
    let elevation: Vec<Vec<u32>> = grid.iter().map(|l| 
        l.iter().map(|c| *c as u32).collect()).collect();
    
    let open: Vec<Node> = Vec::new();
    let closed: Vec<Node> = Vec::new();
    let mut g = 'S';
    while g != 'E' {

    }


    for i in 0..elevation.len() {

        println!("{:?}", elevation[i]);
    }
    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }

}

