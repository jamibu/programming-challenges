const TIME: [i64; 4] = [53, 83, 72, 88];
const DIST: [i64; 4] = [333, 1635, 1289, 1532];
const TIME2: i64 = 53837288;
const DIST2: i64 = 333163512891532;

fn main() {
    let part1: i64 = (0..TIME.len()).map(|i| calc_from_roots(TIME[i], DIST[i])).product();
    println!("Part 1: {}", part1);
    let part2: i64 = calc_from_roots(TIME2, DIST2);
    println!("Part 2: {}", part2);
}

fn calc_from_roots(time: i64, dist: i64) -> i64 {
    let sq = ((time.pow(2) - 4 * dist) as f64).sqrt();
    let tcharge1 = (-time as f64 + sq) / (-2.0);
    let tcharge2 = (-time as f64 - sq) / (-2.0);

    return tcharge2.ceil() as i64 - tcharge1.floor() as i64 - 1; 
}
