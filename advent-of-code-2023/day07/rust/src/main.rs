use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Hand {
    // cards: (u32, u32, u32, u32, u32),
    cards: Vec<u32>,
    bid: u32,
    rank: (usize, usize),
}

impl Hand {
    fn from_str(line: String, is_part2: bool) -> Hand {
        let (hand_str, bid_str) = line.split_once(' ').expect("Could not split.");
        let cards: Vec<u32> = hand_str
            .chars()
            .map(|x| card_char_to_num(x, is_part2))
            .collect();

        let bid: u32 = bid_str.parse().unwrap();

        let mut counts: HashMap<u32, usize> = HashMap::new();
        for card in &cards {
            add_or_increment(&mut counts, *card);
        }

        let mut rank = rank_hand(&counts);

        if is_part2 && hand_str.contains('J') {
            rank = upgrade_with_joker(rank, counts[&1]);
        }

        return Hand { cards, bid, rank };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            return Some(self.cards.cmp(&other.cards));
        }
        Some(self.rank.cmp(&other.rank))
    }
}

fn main() {
    let fname = "../puzzleInput.txt";
    let is_part2 = true;
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let mut hands: Vec<Hand> = lines
        .map(|x| Hand::from_str(x.unwrap(), is_part2))
        .collect();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let ans: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, x)| x.bid * (i as u32 + 1))
        .sum();

    println!("Ans: {}", ans);
}

fn card_char_to_num(card: char, is_part2: bool) -> u32 {
    let j = if is_part2 { 1 } else { 11 };

    return match card {
        'T' => 10,
        'J' => j,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    };
}

fn add_or_increment(map: &mut HashMap<u32, usize>, card: u32) {
    match map.get(&card) {
        Some(count) => {
            map.insert(card, count + 1);
        }
        None => {
            map.insert(card, 1);
        }
    }
}

fn rank_hand(counts: &HashMap<u32, usize>) -> (usize, usize) {
    let mut rank: (usize, usize) = (0, 0);
    for val in counts.values() {
        let val = *val;
        if val > rank.0 {
            rank = (val, rank.0);
        } else if val > rank.1 {
            rank.1 = val;
        }
    }

    rank
}

fn upgrade_with_joker(mut rank: (usize, usize), num_j: usize) -> (usize, usize) {
    rank = match (rank, num_j) {
        ((5, 0), _) | ((4, 1), _) | ((3, 2), _) => (5, 0),
        ((3, 1), _) | ((2, 2), 2) => (4, 1),
        ((2, 2), 1) => (3, 2),
        ((2, 1), _) => (3, 1),
        ((1, 1), _) => (2, 1),
        _ => panic!("Unexpected hand"),
    };
    return rank;
}
