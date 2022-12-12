use std::fs;


#[derive(Debug)]
struct VisMap {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl VisMap {
    fn new(width: usize, height: usize) -> VisMap {
        // Create a new VisMap with all values set to 0
        let map = vec![vec![0; width]; height];
        VisMap {map, width, height}
    }
    fn analyse_grid(&mut self, grid: &Vec<Vec<i32>>) {
        // Analyse the grid and update the VisMap
        for (y, row) in grid.iter().enumerate() {
            self.analyse_row(&row, y);
        }
        for x in 0..self.width {
            let col = grid.iter().map(|c| c[x]).collect();
            self.analyse_col(&col, x);
        }

    }
    fn analyse_row(&mut self, row: &Vec<i32>, y: usize) {
        let mut local_max_left = -1;
        let mut local_max_right = -1;
        
        for x in 0..self.width {
            if row[x] > local_max_left {
                self.map[y][x] += 1;
                local_max_left = row[x];
            }
            if row[self.width - x - 1] > local_max_right {
                self.map[y][self.width - x - 1] += 1;
                local_max_right = row[self.width - x - 1];
            }
        }
    }
    fn analyse_col(&mut self, col: &Vec<i32>, x: usize) {
        let mut local_max_top = -1;
        let mut local_max_bottom = -1;
        
        for y in 0..self.height {
            if col[y] > local_max_top {
                self.map[y][x] += 1;
                local_max_top = col[y];
            }
            if col[self.height - y - 1] > local_max_bottom {
                self.map[self.height - y - 1][x] += 1;
                local_max_bottom = col[self.height - y - 1];
            }
        }
    }
    fn count_visible(&self) -> u32 {
        // Count the number of visible trees 
        let mut count = 0;
        for row in self.map.iter() {
            for tree in row.iter() {
                if *tree > 0 {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let grid: Vec<Vec<i32>> = contents.strip_suffix("\n").unwrap().split("\n")
        .map(|l| l.chars()
             .map(|c| c.to_digit(10).unwrap() as i32)
             .collect()
        ).collect();
    
    let mut vis_map = VisMap::new(grid[0].len(), grid.len());

    vis_map.analyse_grid(&grid);
    for row in vis_map.map.iter() {
        println!("{:?}", row.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""));
    }
    println!("{:?}", vis_map.count_visible());
}

fn check_visibility(Vec<Vec<i32>>) {}
