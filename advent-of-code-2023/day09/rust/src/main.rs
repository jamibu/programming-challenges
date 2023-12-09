use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let part1 = lines.map(|x| parse_line(x.expect("No lines")));
}

fn parse_line(line: String) -> Vec<u32> {
    return line.trim().split_whitespace().map(|x| x.parse::<u32>().expect("Not a number")).collect();
}
