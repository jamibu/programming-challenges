use std::fs;
use std::collections::HashSet;

fn main() {
    let filename = "puzzleInput.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match solve(&contents, 4) {
        Some(result) => println!("Result: {}", result),
        None => println!("No solution found"),
    };

    match solve(&contents, 14) {
        Some(result) => println!("Result: {}", result),
        None => println!("No solution found"),
    };
}

fn solve(contents: &str, num_unique: usize) -> Option<usize> {
    contents.as_bytes()
        .windows(num_unique)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() == num_unique)
        .map(|(i, _)| i + num_unique)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let example: String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(Some(7), solve(&example, 4));
    }

    #[test]
    fn part2() {
        let example: String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(Some(19), solve(&example, 14));
    }
}
