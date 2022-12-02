use std::io::{BufRead, BufReader};
use std::fs::File;


#[derive(Debug, Clone, Copy)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum RoundResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

fn main() {
    // Getting the strats
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    let mut total_score_1 = 0;
    let mut total_score_2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if let [strat1, strat2] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let opponent_move = match strat1 {
                "A" => Moves::Rock,
                "B" => Moves::Paper,
                "C" => Moves::Scissors,
                _ => panic!("Invalid move"),
            };

            let our_move = match strat2 { 
                "X" => Moves::Rock,
                "Y" => Moves::Paper,
                "Z" => Moves::Scissors,
                _ => panic!("Invalid move"),
            };
            total_score_1 += evaluate_round_1(&opponent_move, &our_move);
            total_score_2 += evaluate_round_2(&opponent_move, strat2);
        } else {
            println!("Invalid input");
        }
    }
    
    println!("Total score 1: {}", total_score_1);
    println!("Total score 2: {}", total_score_2);
}

fn evaluate_round_1(p1_move: &Moves, p2_move: &Moves) -> i32 {
    let result_score = match (p1_move, p2_move) {
        (Moves::Rock, Moves::Rock) => RoundResult::Draw,
        (Moves::Rock, Moves::Paper) => RoundResult::Win,
        (Moves::Rock, Moves::Scissors) => RoundResult::Loss,
        (Moves::Paper, Moves::Rock) => RoundResult::Loss,
        (Moves::Paper, Moves::Paper) => RoundResult::Draw,
        (Moves::Paper, Moves::Scissors) => RoundResult::Win,
        (Moves::Scissors, Moves::Rock) => RoundResult::Win,
        (Moves::Scissors, Moves::Paper) => RoundResult::Loss,
        (Moves::Scissors, Moves::Scissors) => RoundResult::Draw,
    };

    return result_score as i32 + *p2_move as i32;
}

fn evaluate_round_2(p1_move: &Moves, strat: &str) -> i32 {
    let result = match strat {
        "X" => RoundResult::Loss,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Invalid move"),
    };

    let p2_move = match (&result, p1_move) {
        (RoundResult::Win, Moves::Rock) => Moves::Paper,
        (RoundResult::Win, Moves::Paper) => Moves::Scissors,
        (RoundResult::Win, Moves::Scissors) => Moves::Rock,
        (RoundResult::Loss, Moves::Rock) => Moves::Scissors,
        (RoundResult::Loss, Moves::Paper) => Moves::Rock,
        (RoundResult::Loss, Moves::Scissors) => Moves::Paper,
        (RoundResult::Draw, _) => *p1_move,
    };

    return result as i32 + p2_move as i32;
}

