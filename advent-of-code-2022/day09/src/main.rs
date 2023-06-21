use std::io::{BufRead, BufReader};
use std::fs::File;


struct Postition {
    row: i32,
    col: i32 
}


#[derive(Debug)]
enum Move {
    Vertical(i32),
    Horizontal(i32),
}

impl Move {
    fn from_line(line: String) -> Move {
        let movement = line.split_once(" ").expect("Can't split line");
        let dist: i32 = movement.1.parse().expect("Invalid distance");
        match movement.0 {
            "U" => Move::Vertical(dist),
            "D" => Move::Vertical(-dist),
            "R" => Move::Horizontal(dist),
            "L" => Move::Horizontal(-dist),
            _ => panic!("Unhandled Instruction"),
        }
    }
}


fn main() {
    let filename = "example.txt";
    // let filename = "puzzleInput.txt";

    let head_pos = Postition {row: 0, col: 0};
    let tail_pos = Postition {row: 0, col: 0};

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    for line in reader.lines() {
        let movement = Move::from_line(line.expect("Could not get line"));
        println!("{:?}", movement);
    }

}
