use std::fs::File;
use std::io::{self, BufRead};

const NUMS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn main() {
    let fname = "puzzleInput.txt";

    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    // let total_part1 = lines.fold(0u32, |a, x| a + part1(x.unwrap()));
    // println!("{}", total_part1);

    let total_part2 = lines.fold(0u32, |a, x| a + part2(x.unwrap()));
    println!("{}", total_part2);
}

fn part1(line: String) -> u32 {
    let first = line
        .chars()
        .find(|&x| x.is_numeric())
        .expect("No digit found");
    let last = line
        .chars()
        .rev()
        .find(|&x| x.is_numeric())
        .expect("No digit found");

    let first_last = format!("{}{}", first, last);

    return first_last.parse::<u32>().unwrap();
}

fn part2(line: String) -> u32 {
    let mut numbers: Vec<_> = vec![];

    for num in NUMS {
        numbers.append(&mut line.match_indices(num).into_iter().collect::<Vec<_>>())
    }

    let mut min_idx = usize::MAX;
    let mut max_idx = usize::MIN;
    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;
    for (idx, num) in numbers {
        if idx <= min_idx {
            min_idx = idx;
            first = Some(num);
        }
        if idx >= max_idx {
            max_idx = idx;
            last = Some(num);
        }
    }

    let first_index = digit_from_str(first.expect("First digit missing"));
    let last_index = digit_from_str(last.expect("Last digit missing"));

    let first_last = format!("{}{}", first_index, last_index);
    return first_last.parse::<u32>().unwrap();
}

fn digit_from_str(digit_str: &str) -> usize {
    let mut index = NUMS.iter().position(|&x| x == digit_str).unwrap();
    if index > 9 {
        index -= 10
    }

    return index;
}
