use std::{collections::HashSet, error::Error, fs, io::{BufReader, BufRead}};

type Map = Vec<Vec<char>>;
type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Parity {
    L,
    R
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn next(self, c: &char) -> Self {
        return match self {
            Direction::Up => {
                match c {
                    &'|' => Direction::Up,
                    &'7' => Direction::Left,
                    &'F' => Direction::Right,
                    &'S' => Direction::Up,
                    _ => panic!("Shouldn't be here!")
                }
            }
            Direction::Right => {
                match c {
                    &'-' => Direction::Right,
                    &'7' => Direction::Down,
                    &'J' => Direction::Up,
                    _ => panic!("Shouldn't be here!")
                }
            }
            Direction::Down => {
                match c {
                    &'|' => Direction::Down,
                    &'L' => Direction::Right,
                    &'J' => Direction::Left,
                    _ => panic!("Shouldn't be here!")
                }
            }
            Direction::Left => {
                match c {
                    &'-' => Direction::Left,
                    &'L' => Direction::Up,
                    &'F' => Direction::Down,
                    _ => panic!("Shouldn't be here!")
                }
            }
        }
    }
}

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

fn find_start(m: &Map) -> Coord {
    let mut coord = (0, 0);
    for (row, line) in m.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c == &'S' {
                coord = (row, col);
            }
        }
    }
    return coord;
}

fn next_coord<'a>(coord: Coord, direction: Direction, m: &'a Map) -> (Coord, Direction, &'a char) {
    let newcoord = match direction {
        Direction::Up => (coord.0 - 1, coord.1),
        Direction::Right => (coord.0, coord.1+1),
        Direction::Down => (coord.0 + 1, coord.1),
        Direction::Left => (coord.0, coord.1-1),
    };
    let c = &m[newcoord.0][newcoord.1];
    let newdirection = direction.next(c);
    return (newcoord, newdirection, c);
}

fn get_adjacent(coord: Coord, direction: Direction, c: &char, parity: Parity, m: &Map) -> HashSet<Coord> {
    let mut adjcoords: HashSet<Coord> = HashSet::new();
    let (row, col) = coord;

    match c {
        &'|' => match (parity, direction) {
            (Parity::R, Direction::Up) => {adjcoords.insert((row, col+1));},
            (Parity::R, Direction::Down) => {if col > 0 {adjcoords.insert((row, col-1));}},
            (Parity::L, Direction::Up) => {if col > 0 {adjcoords.insert((row, col-1));}},
            (Parity::L, Direction::Down) => {adjcoords.insert((row, col+1));},
            other => panic!("{:?}, {:?}", other.0, other.1),
        }
        &'-' => match (parity, direction) {
            (Parity::R, Direction::Right) => {adjcoords.insert((row+1, col));},
            (Parity::R, Direction::Left) => {if row > 0 {adjcoords.insert((row-1, col));}},
            (Parity::L, Direction::Right) => {if row > 0 {adjcoords.insert((row-1, col));}},
            (Parity::L, Direction::Left) => {adjcoords.insert((row+1, col));},
            other => panic!("{:?}, {:?}", other.0, other.1),
        }
        &'J' => match (parity, direction) {
            (Parity::R, Direction::Up) | (Parity::L, Direction::Left) => {
                adjcoords.insert((row+1, col));
                adjcoords.insert((row, col+1));
            }
            _ => {},
        }
        &'L' => match (parity, direction) {
            (Parity::L, Direction::Up) | (Parity::R, Direction::Right) => {
                adjcoords.insert((row+1, col));
                if col > 0 {
                    adjcoords.insert((row, col-1));
                }
            }
            _ => {},
        }
        &'F' => match (parity, direction) {
            (Parity::L, Direction::Right) | (Parity::R, Direction::Down) => {
                if row > 0 {
                    adjcoords.insert((row-1, col));
                }
                if col > 0 {
                    adjcoords.insert((row, col-1));
                }
            }
            _ => {},
        }
        &'7' => match (parity, direction) {
            (Parity::L, Direction::Down) | (Parity::R, Direction::Left) => {
                if row > 0 {
                    adjcoords.insert((row-1, col));
                }
                adjcoords.insert((row, col+1));
            }
            _ => {},
        }
        &'0' => {},
        other => panic!("{:?}", other),
    }

    let maxrow = m.len();
    let maxcol = m[0].len();
    adjcoords.retain(|(row, col)| row < &maxrow && col < &maxcol);
    return adjcoords;

}

fn get_neighbors(coord: Coord, m: &Map) -> Vec<Coord> {
    let maxrow = m.len() - 1;
    let maxcol = m[0].len() - 1;
    let (row, col) = coord;
    let mut neighbors: Vec<Coord> = Vec::new();
    if row > 0 {
        neighbors.push((row-1, col));
    }
    if row < maxrow {
        neighbors.push((row+1, col));
    }
    if col > 0 {
        neighbors.push((row, col-1));
    }
    if col < maxcol {
        neighbors.push((row, col+1));
    }
    return neighbors;
}

fn dfs(start: Coord, m: &Map, loop_coords: &HashSet<Coord>, all_coords: &HashSet<Coord>) -> HashSet<Coord> {
    let mut coords: HashSet<Coord> = HashSet::new();
    let mut stack: Vec<Coord> = vec![start];
    while let Some(coord) = stack.pop() {
        coords.insert(coord);
        let mut neighbors = get_neighbors(coord, &m);
        neighbors.retain(|n| !loop_coords.contains(n) &&
                                              !coords.contains(n) &&
                                              !all_coords.contains(n));
        stack.extend(neighbors);
    }
    return coords;
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(open_file("input.txt", "day10/"));

    let m: Map = buf.lines()
        .map(|line| {
            line.unwrap().chars().collect()
        }).collect();

    let mut coord = find_start(&m);
    let mut direction = Direction::Up;
    let mut c = &'0';
    let mut coords: HashSet<Coord> = HashSet::new();
    let mut rights: HashSet<Coord> = HashSet::new();
    let mut lefts: HashSet<Coord> = HashSet::new();
    while c != &'S' {
        coords.insert(coord);
        let left = get_adjacent(coord, direction, c, Parity::L, &m);
        let right = get_adjacent(coord, direction, c, Parity::R, &m);
        (coord, direction, c) = next_coord(coord, direction, &m);
        lefts.extend(left);
        rights.extend(right);
    }

    lefts.retain(|coord| !coords.contains(coord));
    rights.retain(|coord| !coords.contains(coord));
    let lefts = lefts;
    let rights = rights;

    let mut all_lefts: HashSet<Coord> = HashSet::new();
    for left in lefts {
        let new_lefts = dfs(left, &m, &coords, &all_lefts);
        all_lefts.extend(new_lefts);
    }

    let mut all_rights: HashSet<Coord> = HashSet::new();
    for right in rights {
        let new_rights = dfs(right, &m, &coords, &all_rights);
        all_rights.extend(new_rights);
    }

    println!("{}", all_lefts.len());
    println!("{}", all_rights.len());
    return Ok(());
}