use std::fs;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_string(string: &str) -> Instruction {
        let parts = string.split_whitespace().collect::<Vec<&str>>();
        Instruction {
            amount: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap(),
            to: parts[5].parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let (initial_stacks, instructions) = contents.split_once("\n\n").expect("No split point");
    let instructions: Vec<Instruction> = instructions
        .strip_suffix("\n").unwrap()
        .split("\n")
        .map(|x| Instruction::from_string(x))
        .collect();
    let stacks = parse_stacks(initial_stacks);

    println!("{}", solve(stacks.clone(), &instructions, true));
    println!("{}", solve(stacks, &instructions, false));
}

fn parse_stacks(initial_stacks: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = initial_stacks
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let nums = rows.pop().unwrap();

    for (i, num) in nums.iter().enumerate() {
        if num != &' ' {
            let stack: Vec<char> = rows.iter().rev().map(|x| x[i]).filter(|x| x != &' ').collect();
            stacks.push(stack);
        }
    }
    return stacks
}

fn solve(
    mut stacks: Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
    crane9001: bool,
) -> String {
    let mut to_move: Vec<char> = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.amount {
            to_move.push(stacks[instruction.from - 1].pop().unwrap());
        }
        if crane9001 {
            to_move.reverse();
        }
        stacks[instruction.to - 1].append(&mut to_move);
    }

    return stacks.iter().filter_map(|x| x.last()).collect::<String>() 
}
