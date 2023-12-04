use std::fs;
use std::error::Error;


struct Coord {
    x: usize,
    y: usize,
}


fn main() -> Result<(), Box<dyn Error>> {
    let fname = "../example.txt";
    let input_str: String = fs::read_to_string(fname)?;
    // Parsing the string at once since we'll need to search around in a grid
    let grid: Vec<Vec<char>> = input_str 
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    part1(&grid);

    Ok(())
}

fn part1(rows: &Vec<Vec<char>>) {
    for (y, row) in rows.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            if row[x].is_numeric() || row[x] == '0'{
                x += 1;
                continue;
            }

            let last_x = row[x..]
                .iter()
                .position(|&r| !r.is_numeric())
                .unwrap() + x - 1;

            // check_surrounding(rows, x, end_x, y);
            // Go to next char after the number
            x += last_x + 1;
        }
    }
}

fn get_surrounding(first_x: usize, last_x: usize, y: usize, rows: usize, cols: usize) {
    if first_x == 0 {

    }

    if last_x == 

    let start_x = first_x - 1 * (first_x > 0) as usize;
    let end_x = last_x + 1 * (last_x > 0) as usize;
    
    let mut surrounding: Vec<usize> = vec![];

    if y > 0 {
        let y_top = y - 1;
        surrounding.append((start_x..end_x).collect());
    }

}
