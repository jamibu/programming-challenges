use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Dirs {
    stack: Vec<String>,
    size: HashMap<String, i32>
}

impl Dirs {
    fn parse_cd(&mut self, line: String) {
        let dir = line.split_whitespace().last().unwrap();
        
        if dir == ".." {
            self.stack.pop();
        } else {
            self.stack.push(dir.to_string());
        }
    }    
    fn parse_file_size(&mut self, line: String) {
        let file_size: i32 = line.split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        for dir in self.stack.iter() {
            match self.size.get_mut(dir) {
                Some(val) => *val += file_size,
                None => {self.size.insert(dir.to_string(), file_size);},
            }
        }
    }
}

fn main() {
    let mut dirs = Dirs {
        stack: Vec::new(), 
        size: HashMap::new()
    };
    
    // let filename = "example.txt";
    let filename = "puzzle_input.txt";

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" || line.starts_with("$ ls") || line.starts_with("dir") {
            continue
        }
        
        if line.starts_with("$ cd") {
            dirs.parse_cd(line);
        } else {
            dirs.parse_file_size(line)
        }
    }
    
    solve1(dirs)
    
}

fn solve1(dirs: Dirs) {
    let mut solution: i32 = 0;
    for (_, v) in dirs.size {
        if v <= 100000 {
            solution += v;
        };
    }
    
    println!("{:?}", solution);
}
