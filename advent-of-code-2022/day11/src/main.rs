use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    variable: Variable,
    divider: u32,
    if_true: u32,
    if_false: u32,
    num_inspected: u32,
}

impl Monkey {
    fn inpect_items(&mut self) -> Vec<(u32, u32)> {
        let mut throws: Vec<(u32, u32)> = vec!(); 
        for item in &self.items {
            println!("item: {}", item);
            let new_item = self.evaluate(item);
            println!("Worry becomes: {}", new_item);
            let new_item = new_item / 3;
            println!("Divide by 3: {}", new_item);
            let throw_to = self.test(&new_item);
            println!("Throw to: {}", throw_to);
            throws.push((throw_to, new_item));
            self.num_inspected += 1;
        };
        return throws
    }
    fn evaluate(&self, item: &u32) -> u32 {
        let val = match self.variable {
            Variable::Value(val) => val,
            Variable::Old => *item,
        };
        match self.operation {
            Operation::Add => item + val,
            Operation::Subtract => item - val,
            Operation::Multiply => item * val,
            Operation::Divide => item / val,
        }

    }
    fn test(&self, item: &u32) -> u32 {
        if item % self.divider == 0 {
            return self.if_true;
        } else {
            return self.if_false;
        }
    }
    fn clear_items(&mut self) {
        self.items = vec!();
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
    Value(u32),
    Old,
}
 
fn main() {
    // let filename = "puzzleInput.txt";
    let filename = "example.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let monkeys = parse_monkeys(&contents);

    part1(monkeys);
}

fn parse_monkeys(content: &str) -> HashMap<u32, Monkey> {
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();

    for monkey in content.split("\n\n") {
        let lines = monkey.split("\n").collect::<Vec<&str>>();

        let id: u32 = lines[0]
            .strip_suffix(":").unwrap()
            .split(" ").last().unwrap()
            .parse().unwrap();

        let items: Vec<u32> = lines[1]
            .split(": ").last().unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        
        let operations: Vec<&str> = lines[2].split_whitespace().collect();
        let variable = match operations[5] {
            "old" => Variable::Old,
            _ => Variable::Value(operations[5].parse::<u32>().unwrap()),
        };
        let operation = match operations[4] {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => panic!("Unknown operation"),
        };
        
        // Create the test case that determines who the monkey throws to
        let divider: u32 = lines[3].split("by ").last().unwrap().parse().unwrap();
        let if_true: u32 = lines[4].split("monkey ").last().unwrap().parse().unwrap();
        let if_false: u32 = lines[5].split("monkey ").last().unwrap().parse().unwrap();
        
        let monkey = Monkey {
            items,
            operation,
            variable,
            divider,
            if_true,
            if_false,
            num_inspected: 0,
        };
        monkeys.insert(id, monkey);
    }
    
    return monkeys
}

fn part1(mut monkeys: HashMap<u32, Monkey>) {
    for _ in 0..20 {
        // Run round for all items
        let mut changes: Vec<Vec<(u32, u32)>> = vec!();
        for (_, monkey) in &mut monkeys {
            println!("Monkey: {:?}", monkey);
            changes.push(monkey.inpect_items());
            monkey.clear_items();
        };
        
        // Apply changes
        for changeset in changes {
            for (id, new_item) in changeset {
                monkeys.get_mut(&id).unwrap().items.push(new_item);
            }
        }
    }
    
    for (id, monkey) in &monkeys {
        println!("Monkey {} inspected {} items", id, monkey.num_inspected);
    }
}   


