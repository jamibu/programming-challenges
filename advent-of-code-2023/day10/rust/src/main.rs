use std::fs::File;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;
type Coord = (usize, usize);

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Movement {
    x: usize,
    y: usize,
    dir: Direction,
}

fn main() {
    let file = File::open("../example.txt").expect("Could not open file");
    let lines = io::BufReader::new(file).lines();

    let maze: Grid = lines.map(|x| x.unwrap().chars().collect()).collect();
    let mut visited: Visited = vec![vec![false; maze[0].len()]; maze.len()];

    let ylim = visited.len() - 1;
    let xlim = visited[0].len() - 1;

    let (y, x) = find_start(&maze);
    visited[y][x] = true;

    let mut moves: Vec<Movement> = vec![];
    let dir = Direction::North;
    moves.push(Movement { x, y, dir });
    let dir = Direction::South;
    moves.push(Movement { x, y, dir });
    let dir = Direction::East;
    moves.push(Movement { x, y, dir });
    let dir = Direction::West;
    moves.push(Movement { x, y, dir });

    let mut count: Option<usize> = None;
    for first_move in moves {
        count = search_pipe(&maze, first_move, xlim, ylim);
    }
    dbg!(count);
    println!("Part 1: {}", count.expect("No count found"))
}

fn search_pipe(maze: &Grid, mut movement: Movement, xlim: usize, ylim: usize) -> Option<usize> {
    let mut count: usize = 0;
    loop {
        // dbg!(maze[movement.y][movement.x]);
        dbg!(&movement);
        let (inbound, new_coords) = apply_movement(&movement, xlim, ylim);
        if !inbound {
            return None;
        }

        let c = maze[new_coords.0][new_coords.1];
        dbg!(c);

        if c == 'S' {
            dbg!(count);
            return Some(count);
        }

        let next = next_direction(c, &movement.dir);
        match next {
            Some(val) => {
                movement = Movement {
                    dir: val,
                    y: new_coords.0,
                    x: new_coords.1,
                }
            }
            None => return None,
        }

        count += 1
    }
}

fn find_start(maze: &Grid) -> (usize, usize) {
    for (i, row) in maze.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            if *pipe == 'S' {
                return (i, j);
            }
        }
    }
    panic!("Start not found")
}

fn next_direction(pipe: char, last_movement: &Direction) -> Option<Direction> {
    return match (pipe, last_movement) {
        ('|', Direction::North) => Some(Direction::North),
        ('-', Direction::East) => Some(Direction::East),
        ('-', Direction::West) => Some(Direction::West),
        ('L', Direction::South) => Some(Direction::East),
        ('L', Direction::West) => Some(Direction::North),
        ('J', Direction::South) => Some(Direction::West),
        ('J', Direction::East) => Some(Direction::North),
        ('F', Direction::North) => Some(Direction::East),
        ('F', Direction::West) => Some(Direction::South),
        ('7', Direction::North) => Some(Direction::West),
        ('7', Direction::East) => Some(Direction::South),
        _ => None,
    };
}

fn apply_movement(movement: &Movement, xlim: usize, ylim: usize) -> (bool, Coord) {
    let x = movement.x as isize;
    let y = movement.y as isize;
    let (inbound, new_coord) = match movement.dir {
        Direction::North => (movement.y > 0, (y - 1, x)),
        Direction::South => (movement.y < ylim, (y + 1, x)),
        Direction::East => (movement.x < xlim, (y, x + 1)),
        Direction::West => (movement.x > 0, (y, x - 1)),
    };

    return (inbound, (new_coord.0 as usize, new_coord.1 as usize));
}
