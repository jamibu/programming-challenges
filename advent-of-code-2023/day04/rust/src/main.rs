use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const NUM_CARDS: usize = 209;

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut cards: [usize; NUM_CARDS] = [1; NUM_CARDS];
    for (i, line) in lines.enumerate() {
        let wins: usize = parse_card(line.unwrap());
        if wins > 0 {
            part1 += 2u32.pow(wins as u32 - 1)
        }

        part2 += process_card(&mut cards, wins, i);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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

fn process_card(cards: &mut [usize; NUM_CARDS], wins: usize, idx: usize) -> usize {
    for j in (idx + 1)..=(idx + wins) {
        cards[j] += cards[idx]
    }
    return cards[idx];
}
