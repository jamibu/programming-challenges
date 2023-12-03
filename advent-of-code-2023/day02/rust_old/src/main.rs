use std::fs::File;
use std::io::{self, BufRead};

const LIMITS: [usize; 3] = [12, 13, 14];

fn main() {
    println!("Hello, world!");
    solve1("puzzleInput.txt")
}

fn solve1(fname: &str) {
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let result = lines.fold(0, |a, x| a + parse_game(x.expect("No line")));
    println!("{}", result);
}

fn solve2(fname: &str) {
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let result = lines.fold(0, |a, x| a + parse_game(x.expect("No line")));
    println!("{}", result);
}

fn parse_game(line: String) -> u32 {
    let line_split: Vec<&str> = line.split(":").collect();

    if is_possible(line_split[1]) {
        let game_id = line_split[0]
            .split_whitespace()
            .last()
            .expect("Game ID missing");

        return game_id.parse().expect("Failed");
    }

    return 0;
}

fn is_possible(game: &str) -> bool {
    for round in game.split(";") {
        for cube in round.split(",") {
            let cube_split: Vec<&str> = cube.split_whitespace().collect();
            let cube_count: usize = cube_split[0].parse().expect("Failed");
            let cube_colour: &str = cube_split[1].trim();

            let possible = match cube_colour {
                "red" => cube_count <= LIMITS[0],
                "green" => cube_count <= LIMITS[1],
                "blue" => cube_count <= LIMITS[2],
                _ => panic!("Unexpected cube"),
            };

            if !possible {
                return false;
            }
        }
    }

    return true;
}

fn find_min(game: &str) -> (usize, usize, usize) {
    for round in game.split(";") {
        for cube in round.split(",") {
            let cube_split: Vec<&str> = cube.split_whitespace().collect();
            let cube_count: usize = cube_split[0].parse().expect("Failed");
            let cube_colour: &str = cube_split[1].trim();

            let possible = match cube_colour {
                "red" => cube_count <= LIMITS[0],
                "green" => cube_count <= LIMITS[1],
                "blue" => cube_count <= LIMITS[2],
                _ => panic!("Unexpected cube"),
            };
        }
    }
}
