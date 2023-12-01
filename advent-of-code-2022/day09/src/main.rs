use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;


#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32 
}

impl Position {
    fn make_move(&mut self, movement: Move) {
        match movement {
            Move::Up(_) => self.row += 1,
            Move::Down(_) => self.row -= 1,
            Move::Right(_) => self.col += 1,
            Move::Left(_) => self.col -= 1,
        }
    }

    fn follow(&self, leader: &Position) -> Position {
        let mut new_position =  Position {row: 0, col: 0};

        if leader == self {
            new_position.row = self.row;
            new_position.col = self.col;
        }


        // Branchless solution. Add the values
        let rows_equal = (leader.row == self.row) as i32;
        let cols_equal = leader.col == self.col as i32;
        let up = rows_equal * ((leader.col - self.col == 2) as i32) + !rows_equal * !cols_equal * ((leader.col - self.col == 1) as i32);


        let two_above = ;
        let two_below = leader.col - self.col == -2 as i32;


        let two_left = leader.row - self.row == 2 as i32;
        let two_left = leader.row - self.row == -2 as i32;

        if  {
            new_position.col = self.col + (leader.col - self.col == 2) as i32;
            new_position.col = self.col - (leader.col - self.col == -2) as i32;

        }
        
        if leader.col == self.col {

        }
    }
}


#[derive(Debug)]
enum Move {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

impl Move {
    fn from_line(line: String) -> Move {
        let movement = line.split_once(" ").expect("Can't split line");
        let dist: usize = movement.1.parse().expect("Invalid distance");
        match movement.0 {
            "U" => Move::Up(dist),
            "D" => Move::Down(dist),
            "R" => Move::Right(dist),
            "L" => Move::Left(dist),
            _ => panic!("Unhandled Instruction"),
        }
    }
}


fn main() {
    let filename = "example.txt";
    // let filename = "puzzleInput.txt";

    let mut head_pos = Position {row: 0, col: 0};
    let mut tail_pos = Position {row: 0, col: 0};

    let mut tail_visited: HashSet<Position> = HashSet::new();
    tail_visited.insert(tail_pos);

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    for line in reader.lines() {
        let movement = Move::from_line(line.expect("Could not get line"));
        head_pos.make_move(movement);
        println!("{:?}", head_pos);
    }

}
