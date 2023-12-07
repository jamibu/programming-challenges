use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "../example.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let hands: Vec<(Vec<u32>, u32)> = lines.map(|x| parse_hand(x.unwrap())).collect();
    println!("{:?}", hands);
}

fn parse_hand(line: String) -> (Vec<u32>, u32) {
    let (hand_str, bid_str) = line.split_once(' ').expect("Could not split.");
    let hand: Vec<u32> = hand_str.chars().map(|x| card_char_to_num(x)).collect();
    let bid: u32 = bid_str.parse().unwrap();

    let counts: HashMap<u32, usize> = HashMap::new();
    for card in &hand {}

    return (hand, bid);
}

fn card_char_to_num(card: char) -> u32 {
    return match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    };
}
