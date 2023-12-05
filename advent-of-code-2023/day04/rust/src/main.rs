use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();
    let matches: Vec<usize> = lines.map(|x| parse_card(x.unwrap())).collect();

    let part_1: u32 = matches
        .iter()
        .filter(|x| **x > 0)
        .fold(0u32, |a, x| a + 2u32.pow(*x as u32 - 1));
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part2(&matches));
}

fn parse_card(line: String) -> usize {
    let card: Vec<&str> = line.split(':').last().unwrap().split('|').collect();
    let winners: HashSet<u32> = card[0]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    return card[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|x| winners.contains(x))
        .count();
}

fn part2(matches: &Vec<usize>) -> usize {
    let mut cards: VecDeque<usize> = VecDeque::from((0..matches.len()).collect::<Vec<usize>>());
    let mut total_cards: usize = 0;
    while !cards.is_empty() {
        let card_idx = cards.pop_front().unwrap();
        for idx in 1..=matches[card_idx] {
            cards.push_back(card_idx + idx)
        }
        total_cards += 1
    }

    return total_cards;
}
