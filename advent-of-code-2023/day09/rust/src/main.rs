use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "../puzzleInput.txt";
    let file = File::open(fname).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let (part1, part2) = lines
        .map(|x| parse_line(x.expect("No lines")))
        .map(|x| reduce_diffs(x))
        .fold((0i32, 0i32), |(a, b), (x, y)| (a + x, b + y));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_line(line: String) -> Vec<i32> {
    return line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Not a number"))
        .collect();
}

fn reduce_diffs(nums: Vec<i32>) -> (i32, i32) {
    let mut count = 0;
    let mut rows: Vec<Vec<i32>> = vec![];
    rows.push(nums);
    while !rows[count].iter().all(|x| *x == 0) {
        let num_rows = rows[count].len();
        let new_row: Vec<i32> = (0..(num_rows - 1))
            .map(|i| rows[count][i + 1] - rows[count][i])
            .collect();

        rows.push(new_row);
        count += 1;
    }

    let end = rows
        .iter()
        .rev()
        .map(|x| x[x.len() - 1])
        .fold(0i32, |a, b| a + b);

    let start = rows.iter().rev().map(|x| x[0]).fold(0i32, |a, b| b - a);

    return (end, start);
}
