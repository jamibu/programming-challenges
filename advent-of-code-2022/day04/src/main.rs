use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    let mut complete_overlap = 0;
    let mut any_overlap = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue
        }

        let assignments: Vec<i32> = line
            .split_terminator(|c| c == ',' || c == '-')
            .map(|s| {s.parse().unwrap()})
            .collect();

        if is_complete_overlap(&assignments) {
            complete_overlap += 1;
            // Don't need to check for overlap if there is complete overlap
            any_overlap += 1;
        } else if is_any_overlap(&assignments) {
            any_overlap += 1;
        }
    }

    println!("{:?}", complete_overlap);
    println!("{:?}", any_overlap);
}

fn is_complete_overlap(assignments: &Vec<i32>) -> bool {
    let first_in_second = assignments[0] >= assignments[2] && assignments[1] <= assignments[3];
    let second_in_first =  assignments[2] >= assignments[0] && assignments[3] <= assignments[1];
    return first_in_second || second_in_first
}

fn is_any_overlap(assignments: &Vec<i32>) -> bool {
    // When 1 starts after 2 there is no overlap
    // When 1 ends before 2 there is no overlap
    // in all other cases there is some overlap
    return assignments[0] <= assignments[3] && assignments[1] >= assignments[2];
}
