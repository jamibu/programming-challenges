use std::error::Error;
use std::fs;

const OFFSET_X: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const OFFSET_Y: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

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
        return x >= 0 && x <= self.cols as isize && y >= 0 && y <= self.rows as isize;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fname = "../example.txt";
    let input_str: String = fs::read_to_string(fname)?;
    let engine = EngineSchematic::from_string(input_str);

    let mut visited = vec![vec![false; engine.cols]; engine.rows];

    for row in &engine.grid {
        println!("{:?}", row);
    }

    for y in 0..engine.rows {
        for x in 0..engine.cols {
            // Finding symbols instead of numbers
            if engine.grid[y][x].is_numeric() || engine.grid[y][x] == '.' {
                continue;
            }

            let surrounding = find_surrounding(&engine, x as isize, y as isize);

            let number_sum: u32 = surrounding
                .iter()
                .filter_map(|x| visit_number(&engine, &mut visited, *x))
                .sum();
        }
    }

    Ok(())
}

fn find_surrounding(engine: &EngineSchematic, x: isize, y: isize) -> Vec<(usize, usize)> {
    let surrrounding: Vec<(usize, usize)> = (0..8)
        .map(|i| (x + OFFSET_X[i], y + OFFSET_Y[i]))
        .filter(|(x, y)| {
            engine.within_grid(*x, *y) && engine.grid[*y as usize][*x as usize].is_numeric()
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect();

    return surrrounding;
}

fn visit_number(
    engine: &EngineSchematic,
    visited: &mut Vec<Vec<bool>>,
    coord: (usize, usize),
) -> Option<u32> {
    let (x, y) = coord;

    if visited[y][x] {
        return None;
    }

    println!("{}", engine.grid[y][0..x].iter().collect::<String>());
    let start = x - engine.grid[y][0..x]
        .iter()
        .rev()
        .position(|x| !x.is_numeric())
        .unwrap();
    let end = x + engine.grid[y][x..]
        .iter()
        .position(|x| !x.is_numeric())
        .unwrap();

    println!("{} {}", start, end);
    println!(
        "{:?}",
        engine.grid[y][start..end].iter().collect::<Vec<&char>>()
    );
    let number_string: String = engine.grid[y][start..end].iter().collect();
    let number: u32 = number_string.parse().unwrap();

    for x_v in start..end {
        visited[y][x_v] = true
    }

    return Some(number);
}
