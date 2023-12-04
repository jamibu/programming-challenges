use std::fs;
use std::error::Error;


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
    let visited = Vec<Vec<bool>> = vec!()

    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
        }
    }
}

