use std::fs;

fn main() {
    // Read the data for the problem
    // let file = "exampleData.txt";
    let file = "puzzleInput.txt";
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    // Split the data in by elf (there is a blank line between elves) 
    let elves = contents.split("\n\n");

    // Will calculate for all elves and sort to find the highest
    let mut elf_calories: Vec<i32> = vec!();
    
    // Each sub string in elves are rows of calorie values carried by an elf
    for elf in elves {
        let calories = calc_cals(elf);
        elf_calories.push(calories)
    }

    elf_calories.sort();
    elf_calories.reverse();

    println!("Max calories\n{}", elf_calories[0]);
    println!("Sum of top 3\n{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}

fn calc_cals(elf: &str) -> i32 {
    let calories = elf.split("\n");

    let mut total_calories = 0;
    for cal in calories {
        if cal == "" {
            continue
        }
        total_calories += cal.parse::<i32>().unwrap()
    }

    return total_calories
}

