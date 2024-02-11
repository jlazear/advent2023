use std::{error::Error, fs, io::{BufReader, BufRead}};

type Map = Vec<Vec<char>>;
type Coord = (usize, usize);

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

fn main() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(open_file("input.txt", "day10/"));

    let m: Map = buf.lines()
        .map(|line| {
            line.unwrap().chars().collect()
        }).collect();

    let mut coord = find_start(&m);
    let mut direction = Direction::Up;
    let mut c = &'0';
    let mut length = 0;
    while c != &'S' {
        (coord, direction, c) = next_coord(coord, direction, &m);
        length += 1;
    }

    println!("{}", length/2);
    return Ok(());
}