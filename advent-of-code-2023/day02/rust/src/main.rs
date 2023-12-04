use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Cube {
    colour: String,
    count: usize,
}

impl Cube {
    fn from_str(cube_str: &str) -> Cube {
        let cube_split: Vec<&str> = cube_str.split_whitespace().collect();
        return Cube {
            colour: cube_split[1].to_string(),
            count: cube_split[0].parse::<usize>().expect("Invalid cube count"),
        };
    }
}

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for (i, line) in lines.enumerate() {
        let game = parse_game(line.expect("No lines"));
        part1 += match game.iter().find(|x| x.count > limits[x.colour.as_str()]) {
            Some(_) => 0,
            None => i + 1,
        };
        part2 += evaluate_game_part2(&game);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_game(game_string: String) -> Vec<Cube> {
    let game_str: &str = game_string.split(":").last().expect("No games.");

    // The rounds don't really matter
    let cubes: Vec<Cube> = game_str
        .replace(";", ",")
        .split(",")
        .map(|x| Cube::from_str(x))
        .collect();

    return cubes;
}

fn evaluate_game_part2(cubes: &Vec<Cube>) -> usize {
    let mut mins = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for cube in cubes {
        let min = mins.get_mut(cube.colour.as_str()).unwrap();
        if cube.count > *min {
            *min = cube.count
        }
    }

    return mins["red"] * mins["green"] * mins["blue"];
}
