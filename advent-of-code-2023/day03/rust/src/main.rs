use std::error::Error;
use std::fs;

const OFFSET_X: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const OFFSET_Y: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    visited_number: Vec<Vec<bool>>,
}

impl Grid {
    fn from_string(input_str: String) -> Grid {
        let grid: Vec<Vec<char>> = input_str
            .split('\n')
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();
        let visited_number = vec![vec![false; cols]; rows];

        return Grid {
            grid,
            rows,
            cols,
            visited_number,
        };
    }

    fn get_surrounding(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let surrounding: Vec<(isize, isize)> = (0..8)
            .map(|i| (x + OFFSET_X[i], y + OFFSET_Y[i]))
            .filter(|(x, y)| self.within_grid(*x, *y))
            .collect();

        return surrounding
    }

    fn

    fn within_grid(&self, x: isize, y: isize) -> bool {
        return x > 0 && x < self.cols as isize && y > 0 && y < self.rows as isize;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fname = "../example.txt";
    let input_str: String = fs::read_to_string(fname)?;
    // Parsing the string at once since we'll need to search around in a grid
    let grid = Grid::from_string(input_str);

    for (i, row) in grid.grid.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if point.is_numeric() || *point == '.' {
                continue;
            }
        }
    }

    Ok(())
}
