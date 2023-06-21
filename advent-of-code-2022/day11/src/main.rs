use std::{fs, collections::VecDeque};


struct Monkeys {
    monkeys: Vec<Monkey>,
    items: Vec<usize>,
}


impl Monkeys {
    fn parse(content: &str) -> Monkeys {
        let mut monkeys: Vec<Monkey> = vec!();
        let mut items: Vec<usize> = vec!();

        for monkey in content.split("\n\n") {
            let lines = monkey.split("\n").collect::<Vec<&str>>();

            let holding: VecDeque<usize> = lines[1]
                .rsplit_once(": ").expect("Count not parse items line").1
                .split(", ")
                .map(|x| { items.push(x.parse().unwrap()); items.len() - 1 })
                .collect();

            let operations: Vec<&str> = lines[2].split_whitespace().collect();
            let variable = match operations[5] {
                "old" => Variable::Old,
                _ => Variable::Value(operations[5].parse::<usize>().unwrap()),
            };
            let operation = match operations[4] {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("Unknown operation"),
            };
            
            // Create the test case that determines who the monkey throws to
            let divider: usize = lines[3].rsplit_once("by ").unwrap().1.parse().unwrap();
            let if_true: usize = lines[4].rsplit_once("monkey ").unwrap().1.parse().unwrap();
            let if_false: usize = lines[5].rsplit_once("monkey ").unwrap().1.parse().unwrap();
            
            let monkey = Monkey {
                items: holding,
                operation,
                variable,
                divider,
                if_true,
                if_false,
                num_inspected: 0,
            };
            monkeys.push(monkey);
        }
        
        return Monkeys { monkeys, items }
    }
    
    fn inspect(&mut self, id: usize, divider: usize, modulo: bool) {
        let item = self.monkeys[id].items.pop_front().unwrap();
        let (throw_to, new_item_val) = self.monkeys[id].inpect_item(&self.items[item], divider, modulo);
        self.items[item] = new_item_val;

        self.monkeys[throw_to].items.push_back(item);
    }
}


#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    variable: Variable,
    divider: usize,
    if_true: usize,
    if_false: usize,
    num_inspected: usize,
}


impl Monkey {
    fn inpect_item(&self, item: &usize, worry_divider: usize, modulo: bool) -> (usize, usize) {
        // println!("item: {}", item);
        let mut new_item = self.evaluate(&item);
        // println!("Worry becomes: {}", new_item);
        if modulo {
            new_item = new_item % worry_divider;
        } else {
            new_item = new_item / worry_divider;
        }

        // println!("Divide by 3: {}", new_item);
        let throw_to = self.test(&new_item);
        // println!("Throw to: {}", throw_to);
        return (throw_to as usize, new_item)
    }
    fn evaluate(&self, item: &usize) -> usize {

        match (&self.operation, &self.variable) {
            (Operation::Add, Variable::Value(val)) => item + val,
            (Operation::Add, Variable::Old) => item + item,
            (Operation::Subtract, Variable::Value(val)) => item - val,
            (Operation::Subtract, Variable::Old) => item - item,
            (Operation::Multiply, Variable::Value(val)) => item * val,
            (Operation::Multiply, Variable::Old) => item * item,
            (Operation::Divide, Variable::Value(val)) => item * val,
            (Operation::Divide, Variable::Old) => item / item,
        }
    }
    fn test(&self, item: &usize) -> usize {
        if item % self.divider == 0 {
            return self.if_true;
        } else {
            return self.if_false;
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Variable {
    Value(usize),
    Old,
}
 
fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let monkeys = Monkeys::parse(&contents);
    part1(monkeys);

    let monkeys = Monkeys::parse(&contents);
    part2(monkeys);
}

fn part1(mut monkeys: Monkeys) {
    for _ in 0..20 {
        for i in 0..monkeys.monkeys.len() {
            while monkeys.monkeys[i].items.len() > 0 {
                monkeys.inspect(i, 3, false);
                monkeys.monkeys[i].num_inspected += 1
            }
        }
         
    }
        
    let mut inspected: Vec<usize> =  monkeys.monkeys.iter()
        .map(|x| x.num_inspected)
        .collect();

    inspected.sort();

    println!("Monkey Buisness: {}", inspected[inspected.len()-2] * inspected[inspected.len()-1])
}   

fn part2(mut monkeys: Monkeys) {

    let multiple: usize = monkeys.monkeys.iter().fold(1, |acc, val| acc*val.divider);

    for _ in 0..10000 {
        for i in 0..monkeys.monkeys.len() {
            while monkeys.monkeys[i].items.len() > 0 {
                monkeys.inspect(i, multiple, true);
                monkeys.monkeys[i].num_inspected += 1
            }
        }
         
    }
        
    let mut inspected: Vec<usize> =  monkeys.monkeys.iter()
        .map(|x| x.num_inspected)
        .collect();

    inspected.sort_unstable();
    let ans: usize = inspected.iter().rev().take(2).product();

    println!("Monkey Buisness: {}", ans)
}   

