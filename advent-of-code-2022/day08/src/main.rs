use std::fs;

struct Grid {
    data: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(contents: String) -> Grid {
        let data: Vec<Vec<i32>> = contents.strip_suffix("\n").unwrap().split("\n")
            .map(|l| l.chars()
                 .map(|c| c.to_digit(10).unwrap() as i32)
                 .collect()
            ).collect();
        let width = data[0].len();
        let height = data.len();
        Grid {data, width, height}
    }
}

fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let grid = Grid::new(contents);
    let mut count = 0;
    let mut most_scenic = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            let (visible, scenic_score) = solve(&grid, row, col, grid.data[row][col]);
            if visible {
                count += 1;
            };
            if scenic_score > most_scenic {
                most_scenic = scenic_score;
            };
        }
    }
    println!("{}", count);
    println!("{}", most_scenic);
}

fn solve(grid: &Grid, row: usize, col: usize, tree_height: i32) -> (bool, i32) {
    // Check if the tree at (row, col) is visible from the top
    let mut visible_top = true;
    let mut scenic_score_top = 0;
    for r in (0..row).rev() {
        scenic_score_top += 1;
        if grid.data[r][col] >= tree_height {
            visible_top = false;
            break;
        }
    }
    // Check if the tree at (row, col) is visible from the bottom
    let mut visible_bottom = true;
    let mut scenic_score_bottom = 0;
    for r in (row+1)..grid.height {
        scenic_score_bottom += 1;
        if grid.data[r][col] >= tree_height {
            visible_bottom = false;
            break;
        }
    }
    // Check if the tree at (row, col) is visible from the left
    let mut visible_left = true;
    let mut scenic_score_left = 0;
    for c in (0..col).rev() {
        scenic_score_left += 1;
        if grid.data[row][c] >= tree_height {
            visible_left = false;
            break;
        }
    }
    // Check if the tree at (row, col) is visible from the visible_right
    let mut visible_right = true;
    let mut scenic_score_right = 0;
    for c in (col+1)..grid.width {
        scenic_score_right += 1;
        if grid.data[row][c] >= tree_height {
            visible_right = false;
            break;
        }
    }
    let visible = visible_top || visible_bottom || visible_left || visible_right;
    let scenic_score = scenic_score_top * scenic_score_bottom * scenic_score_left * scenic_score_right;
    println!("{}, {}: {} - {},{},{},{}", row, col, grid.data[row][col], scenic_score_top, scenic_score_left, scenic_score_bottom, scenic_score_right);
    return (visible, scenic_score);
}
