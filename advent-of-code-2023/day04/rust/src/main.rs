use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();
    let part1: u32 = lines.map(|x| parse_card(x.unwrap())).sum();
    println!("Part 1: {}", part1);
}

fn parse_card(line: String) -> u32 {
    let card: Vec<&str> = line.split(':').last().unwrap().split('|').collect();
    let winners: HashSet<u32> = card[0]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let num_match: usize = card[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|x| winners.contains(x))
        .count();
    if num_match == 0 {
        return 0;
    }
    return 2u32.pow(num_match as u32 - 1);
}
