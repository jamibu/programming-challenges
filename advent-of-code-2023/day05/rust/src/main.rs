use std::error::Error;
use std::fs;

struct Almanac {
    seeds: Vec<isize>,
    maps: Vec<ConversionMap>,
}

#[derive(Debug)]
struct ConversionMap {
    destination_start: Vec<isize>,
    source_start: Vec<isize>,
    source_end: Vec<isize>,
    diff: Vec<isize>,
}

impl ConversionMap {
    fn from_str(map_str: &str) -> ConversionMap {
        let mut map = ConversionMap {
            destination_start: vec![],
            source_start: vec![],
            source_end: vec![],
            diff: vec![],
        };

        for row in map_str.split('\n').skip(1) {
            let vals: Vec<isize> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
            map.destination_start.push(vals[0]);
            map.source_start.push(vals[1]);
            map.source_end.push(vals[1] + vals[2]);
            map.diff.push(vals[1] - vals[0]);
        }

        return map;
    }

    fn apply_map(&self, source: isize) -> isize {
        let range: Option<usize> = (0..self.diff.len())
            .find(|x| source >= self.source_start[*x] && source <= self.source_end[*x]);

        let mut destination = source;
        if let Some(i) = range {
            destination -= self.diff[i]
        }

        return destination;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let almanac_string: String = fs::read_to_string("../puzzleInput.txt")?.parse()?;
    let almanac = read_almanac(almanac_string);

    let part1 = almanac
        .seeds
        .iter()
        .map(|x| find_location(&almanac, *x))
        .min()
        .unwrap();

    // Part2
    // Parse seeds into ranges
    // Check intersection of ranges with maps
    // Keep going through the intersections of ranges for each map
    // Take the minimum value from the last map (location)

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}

fn read_almanac(almanac_string: String) -> Almanac {
    let mut almanac_sections = almanac_string
        .strip_suffix('\n')
        .expect("Newline not found")
        .split("\n\n");
    let seeds_str: &str = almanac_sections.next().expect("Empty file.");

    let seeds: Vec<isize> = seeds_str
        .split_whitespace()
        .skip(1) // "seeds: 79..." at the start
        .map(|x| x.parse().unwrap())
        .collect();

    let maps: Vec<ConversionMap> = almanac_sections
        .map(|x| ConversionMap::from_str(x))
        .collect();

    return Almanac { seeds, maps };
}

fn find_location(almanac: &Almanac, seed: isize) -> isize {
    let mut source = seed;
    for map in &almanac.maps {
        source = map.apply_map(source)
    }

    return source;
}
