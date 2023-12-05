use std::collections::HashMap;
use std::fs;

struct EngineSchematic {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl EngineSchematic {
    fn from_string(schematic_string: String) -> EngineSchematic {
        let grid: Vec<Vec<char>> = schematic_string
            .split('\n')
            .map(|x| x.chars().collect::<Vec<char>>())
            .filter(|x| x.len() > 0)
            .collect();
        let rows = grid.len();
        let cols = grid[0].len();

        return EngineSchematic { grid, rows, cols };
    }

    fn within_grid(&self, x: isize, y: isize) -> bool {
        return x >= 0 && x < self.cols as isize && y >= 0 && y < self.rows as isize;
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        return !self.grid[y][x].is_numeric() && self.grid[y][x] != '.';
    }

    fn get_symbol(&self, x: usize, y: usize) -> Symbol {
        return Symbol {
            character: self.grid[y][x],
            x,
            y,
        };
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Symbol {
    character: char,
    x: usize,
    y: usize,
}

fn main() {
    let fname = "../puzzleInput.txt";
    let input_str: String = fs::read_to_string(fname).expect("Could not open file");
    let engine = EngineSchematic::from_string(input_str);
    let mut symbol_hash: HashMap<Symbol, Vec<u32>> = HashMap::new();

    let mut part1 = 0;
    for y in 0..engine.rows {
        let mut x = 0;
        while x < engine.cols {
            if !engine.grid[y][x].is_numeric() {
                x += 1;
                continue;
            }

            let end_x = find_number_end(&engine, x, y);
            let number: u32 = engine.grid[y][x..end_x]
                .iter()
                .collect::<String>()
                .parse()
                .expect("Could not convert part number");

            let symbols = find_symbols(&engine, x as isize, end_x as isize, y as isize);

            if symbols.len() > 0 {
                part1 += number;
                update_symbols(&mut symbol_hash, symbols, number);
            }

            x = end_x;
        }
    }

    let part2: u32 = symbol_hash
        .into_iter()
        .filter(|(key, vals)| key.character == '*' && vals.len() == 2)
        .map(|(_, vals)| vals.iter().product::<u32>())
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_symbols(engine: &EngineSchematic, first_x: isize, last_x: isize, y: isize) -> Vec<Symbol> {
    let start_x = first_x - 1;
    let end_x = last_x + 1;
    let surrounding_coords: Vec<Symbol> = (start_x..end_x)
        .map(|x| (x, y - 1)) // Above number
        .chain([start_x, end_x - 1].into_iter().map(|x| (x, y))) // Either side of number
        .chain((start_x..end_x).map(|x| (x, y + 1))) // Below number
        .filter(|(x, y)| engine.within_grid(*x, *y))
        .filter(|(x, y)| engine.is_symbol(*x as usize, *y as usize))
        .map(|(x, y)| engine.get_symbol(x as usize, y as usize))
        .collect();

    return surrounding_coords;
}

fn update_symbols(hash: &mut HashMap<Symbol, Vec<u32>>, symbols: Vec<Symbol>, number: u32) {
    for symbol in symbols {
        if symbol.character != '*' {
            continue;
        }
        hash.entry(symbol).or_insert(Vec::new()).push(number);
    }
}

fn find_number_end(engine: &EngineSchematic, x: usize, y: usize) -> usize {
    // Determine coord of number end
    let last = engine.grid[y][x..engine.cols]
        .into_iter()
        .position(|x| !x.is_numeric());

    // val is relative to current x position, want to relative to start
    return match last {
        Some(val) => x + val,
        None => engine.cols,
    };
}
