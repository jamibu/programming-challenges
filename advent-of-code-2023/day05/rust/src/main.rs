use std::error::Error;
use std::fs;

struct Almanac {
    seeds: Vec<isize>,
    seed_ranges: Vec<(isize, isize)>,
    maps: Vec<ConversionMap>,
}

#[derive(Debug)]
struct ConversionMap {
    destination: Vec<(isize, isize)>,
    source: Vec<(isize, isize)>,
    diff: Vec<isize>,
}

impl ConversionMap {
    fn from_str(map_str: &str) -> ConversionMap {
        let mut map = ConversionMap {
            destination: vec![],
            source: vec![],
            diff: vec![],
        };

        let mut raw_map: Vec<Vec<isize>> = map_str
            .split('\n')
            .skip(1)
            .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
            .collect();

        raw_map.sort_by(|a, b| a[1].cmp(&b[1]));

        for vals in raw_map {
            map.destination.push((vals[0], vals[0] + vals[2]));
            map.source.push((vals[1], vals[1] + vals[2]));
            map.diff.push(vals[1] as isize - vals[0] as isize);
        }

        return map;
    }

    fn apply_map(&self, source: isize) -> isize {
        let range: Option<usize> = (0..self.diff.len())
            .find(|x| source >= self.source[*x].0 && source <= self.source[*x].1);

        let mut destination: isize = source as isize;
        if let Some(i) = range {
            destination -= self.diff[i]
        }

        return destination as isize;
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
    let mut ranges = range_intersects(&almanac.seed_ranges, &almanac.maps[0]);

    for map in almanac.maps.iter().skip(1) {
        ranges = range_intersects(&ranges, &map);
    }

    let part2 = ranges.iter().map(|x| x.0).min();

    println!("Part 1: {}", part1);
    println!("Part 2: {:?}", part2);

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

    let seed_ranges: Vec<(isize, isize)> = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1]))
        .collect();

    let maps: Vec<ConversionMap> = almanac_sections
        .map(|x| ConversionMap::from_str(x))
        .collect();

    return Almanac {
        seeds,
        seed_ranges,
        maps,
    };
}

fn find_location(almanac: &Almanac, seed: isize) -> isize {
    let mut source = seed;
    for map in &almanac.maps {
        source = map.apply_map(source)
    }

    return source;
}

fn range_intersects(to_check: &Vec<(isize, isize)>, map: &ConversionMap) -> Vec<(isize, isize)> {
    let mut matched: Vec<(isize, isize)> = vec![];
    let mut next: Vec<(isize, isize)> = to_check.clone();

    for (i, source) in map.source.iter().enumerate() {
        let ranges: Vec<(isize, isize)> = next.drain(..).collect();

        for range in ranges {
            // Range is within map source range
            if range.0 >= source.0 && range.1 < source.1 {
                matched.push((range.0 - map.diff[i], range.1 - map.diff[i]));
            // Start of range is within map source range
            } else if range.0 >= source.0 && range.0 < source.1 {
                matched.push((range.0 - map.diff[i], source.1 - map.diff[i]));
                next.push((source.1, range.1));
            // End of range is within map source range
            } else if range.1 >= source.0 && range.1 < source.1 {
                matched.push((source.0 - map.diff[i], range.1 - map.diff[i]));
                next.push((range.0, source.0));
            // Range surrounds map source range
            } else if range.0 < source.0 && range.1 >= source.1 {
                matched.push((source.0 - map.diff[i], source.1 - map.diff[i]));
                next.push((source.1, range.1));
                next.push((range.0, source.0));
            // Range is entirely outside of map source range
            } else {
                next.push(range);
            }
        }
    }

    matched.append(&mut next);

    return matched;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersects() {
        let map = ConversionMap::from_str("seed-to-soil map:\n50 98 2\n52 50 48");

        let ranges1: Vec<(isize, isize)> = vec![(55, 88)];
        let result = range_intersects(&ranges1, &map);
        let expect: Vec<(isize, isize)> = vec![(57, 90)];
        assert_eq!(result, expect);

        let ranges2: Vec<(isize, isize)> = vec![(55, 99)];
        let result = range_intersects(&ranges2, &map);
        let expect: Vec<(isize, isize)> = vec![(57, 100), (50, 51)];
        assert_eq!(result, expect);

        let ranges3: Vec<(isize, isize)> = vec![(55, 100)];
        let result = range_intersects(&ranges3, &map);
        let expect: Vec<(isize, isize)> = vec![(57, 100), (50, 52), (100, 100)];
        assert_eq!(result, expect);

        let ranges4: Vec<(isize, isize)> = vec![(55, 102)];
        let result = range_intersects(&ranges4, &map);
        let expect: Vec<(isize, isize)> = vec![(57, 100), (50, 52), (100, 102)];
        assert_eq!(result, expect);

        let ranges5: Vec<(isize, isize)> = vec![(45, 55)];
        let result = range_intersects(&ranges5, &map);
        let expect: Vec<(isize, isize)> = vec![(52, 57), (45, 50)];
        assert_eq!(result, expect);

        let ranges6: Vec<(isize, isize)> = vec![(96, 102)];
        let result = range_intersects(&ranges6, &map);
        let expect: Vec<(isize, isize)> = vec![(98, 100), (50, 52), (100, 102)];
        assert_eq!(result, expect);

        let ranges7: Vec<(isize, isize)> = vec![(45, 99)];
        let result = range_intersects(&ranges7, &map);
        let expect: Vec<(isize, isize)> = vec![(52, 100), (50, 51), (45, 50)];
        assert_eq!(result, expect);

        let ranges8: Vec<(isize, isize)> = vec![(49, 101)];
        let result = range_intersects(&ranges8, &map);
        let expect: Vec<(isize, isize)> = vec![(52, 100), (50, 52), (100, 101), (49, 50)];
        assert_eq!(result, expect);

        let ranges9: Vec<(isize, isize)> = vec![(101, 111)];
        let result = range_intersects(&ranges9, &map);
        let expect: Vec<(isize, isize)> = vec![(101, 111)];
        assert_eq!(result, expect);
    }
}
