use std::io::{BufRead, BufReader};
use std::fs::File;

use std::collections::{HashSet, HashMap};


fn main() {
    let filename = "puzzleInput.txt";
    // let filename = "example.txt";

    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    let mut char_values: HashMap<char, i32> = HashMap::new();
    
    let mut value = 0;
    for character in 'a'..='z' {
        value += 1;
        char_values.insert(character, value);
    }

    let mut value = 26;
    for character in 'A'..='Z' {
        value += 1;
        char_values.insert(character, value);
    }

    let mut total_values: i32 = 0;
    let mut groups: Vec<Vec<String>> = vec!();
    let mut group: Vec<String> = vec!();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // Each compartment is len/2 long (i.e. split halfway)
        let num_chars = line.len();
        let rucksack_len = num_chars / 2;
        let common_chars = find_common(&line[0..rucksack_len], &line[rucksack_len..num_chars]);
        for c in common_chars {
            total_values += char_values.get(&c).expect("Char not found");
        } 

        if i != 0 && i % 3 == 0 {
            groups.push(group);
            group = vec!();
        }
        group.push(line)
    }

    println!("Part One: {:?}", total_values);

    let mut total_values2 = 0;
    for group in groups.iter() {
        let common1 = find_common(&group[0], &group[1]);
        let common2: Vec<char> = group[2].chars().filter(|c| common1.contains(c)).collect();

        total_values2 += char_values.get(&common2[0]).expect("Could not find char");
    }
    println!("Part Two: {:?}", total_values2);
}


fn find_common(str1: &str, str2: &str) -> HashSet<char> {
    let compartment1: HashSet<char> = str1.chars().collect();
    let common_chars: HashSet<char> = str2.chars()
        .filter(|c| compartment1.contains(c))
        .collect();

    return common_chars
}
