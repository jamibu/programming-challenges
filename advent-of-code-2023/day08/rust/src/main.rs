use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type Nodes = (String, String);

fn main() {
    let fname = "../puzzleInput.txt";
    let (directions, network) = read_file(fname);

    let part1 = follow_nodes(&directions, &network, "AAA", "ZZZ");
    println!("Part 1: {}", part1);
    let part2 = solve_part2(&directions, &network);
    println!("Part 2: {}", part2);
}

fn parse_line(line: String) -> (String, Nodes) {
    let (key, nodes_str) = line.split_once(" = ").expect("Equals missing");
    let nodes = nodes_str[1..(nodes_str.len() - 1)]
        .split_once(", ")
        .expect("Can't split nodes");
    return (key.to_string(), (nodes.0.to_string(), nodes.1.to_string()));
}

fn read_file(fname: &str) -> (Vec<char>, HashMap<String, Nodes>) {
    let file = File::open(fname).expect("Could not open file");
    let mut lines = io::BufReader::new(file).lines();

    let directions: Vec<char> = lines.next().expect("No lines").unwrap().chars().collect();
    let mut network: HashMap<String, Nodes> = HashMap::new();

    for line in lines.skip(1) {
        let (key, nodes) = parse_line(line.unwrap());
        network.insert(key, nodes);
    }

    return (directions, network);
}

fn follow_nodes(
    directions: &Vec<char>,
    network: &HashMap<String, Nodes>,
    start_node: &str,
    end_str: &str,
) -> usize {
    let mut step_idx = 0;
    let mut total_steps = 0;
    let mut node: &str = &start_node;
    while !node.ends_with(end_str) {
        if step_idx >= directions.len() {
            step_idx = 0;
        }

        let dir = directions[step_idx];
        node = match dir {
            'L' => &network[node].0,
            'R' => &network[node].1,
            _ => panic!("Invalid direction"),
        };

        step_idx += 1;
        total_steps += 1;
    }

    return total_steps;
}

fn solve_part2(directions: &Vec<char>, network: &HashMap<String, Nodes>) -> usize {
    let mut steps_per_start: Vec<usize> = vec![];
    for node in network.keys().filter(|x| x.ends_with('A')) {
        steps_per_start.push(follow_nodes(directions, network, node, "Z"));
    }
    let sum: usize = steps_per_start
        .into_iter()
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    return sum;
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}
