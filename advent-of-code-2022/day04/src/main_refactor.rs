use std::io::{BufRead, BufReader};
use std::fs::File;

struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn is_within(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    let mut num_fully_contain = 0;
    let mut num_any_overlap = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue
        }

        let ids: Vec<i32> = line
            .split([',', '-'])
            .map(|s| {s.parse().unwrap()})
            .collect();

        let elf1 = &Assignment{start: ids[0], end: ids[1]};
        let elf2 = &Assignment{start: ids[2], end: ids[3]};
        if elf1.is_within(elf2) || elf2.is_within(elf1) {
            num_fully_contain += 1;
            num_any_overlap += 1;
            continue
        }

        if elf1.overlaps(elf2) {
            num_any_overlap += 1;
        }
    }

    println!("{}", num_fully_contain);
    println!("{}", num_any_overlap);
}

