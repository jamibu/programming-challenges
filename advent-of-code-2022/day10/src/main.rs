use std::io::{BufRead, BufReader};
use std::fs::File;


#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}


fn parse_instruction(line: String) -> (Instruction, i32) {
    let instruction: Instruction;
    let cycles: i32;

    if line.starts_with("addx") {
        let number: i32 = line.rsplit_once(" ")
            .expect("addx missing integer").1
            .parse()
            .expect("addx could not convert integer");
        instruction = Instruction::AddX(number);
        cycles = 2;
    } else if line.starts_with("noop") {
        instruction = Instruction::Noop;
        cycles = 1;
    } else {
        panic!("Unhandled instruction");
    }

    return (instruction, cycles);
}


fn main() {
    // let filename = "example.txt";
    let filename = "puzzleInput.txt";
    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));
    let mut instructions = reader
        .lines()
        .map(|x| parse_instruction(x.expect("Couldn't parse line")));

    let (mut instruction, mut cycles_remaining) = instructions.next()
        .expect("Instructions are empty");

    let mut cycles = 1;
    let mut register_x = 1;
    let mut part1 = 0;

    let mut pixel = 0;
    let mut row = "".to_string();
    let draw = ['.', '#'];
    let mut draw_idx: usize;

    loop {
        if pixel > 39 {
            println!("{}", row);
            pixel = 0;
            row = "".to_string();
        }

        // Update register following completion of previous cycle
        if cycles_remaining == 0 {
            if let Instruction::AddX(x) = instruction {
                register_x += x;
            }

            // Get new Instruction
            (instruction, cycles_remaining) = match instructions.next() {
                Some(val) => val,
                None => break
            };
        } 

        // Strength at every 20, 60, 100...180, 220
        if ((cycles - 20) % 40) == 0 {
            part1 += cycles * register_x;
        }

        // Draw # if sprite overlaps current pixel else .
        draw_idx = (pixel >= register_x - 1 && pixel <= register_x + 1) as usize;
        row.push(draw[draw_idx]);

        cycles_remaining -= 1;
        cycles += 1;
        pixel += 1;
    }
    println!("Part1 {}", part1);
}

