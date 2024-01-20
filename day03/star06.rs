use std::collections::HashSet;
use std::fs;
use std::ops::Index;

#[derive(Debug)]
struct Location {
    row: usize,
    col: usize
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        return Location{row, col};
    }
}

#[derive(Debug)]
struct Map {
    m: Vec<Vec<char>>
}

impl Index<&Location> for Map {
    type Output = char;

    fn index(&self, index: &Location) -> &Self::Output {
        return &self.m[index.row][index.col];
    }
}

impl Index<Location> for Map {
    type Output = char;

    fn index(&self, index: Location) -> &Self::Output {
        return &self.m[index.row][index.col];
    }
}

impl Map {
    fn iter(&self) -> MapIterator {
        return MapIterator{m: &self, index: 0};
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a Vec<char>;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return MapIterator{m: self, index: 0};
    }

}

struct MapIterator<'a> {
    m: &'a Map,
    index: usize
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.m.m.len() {
            let result = Some(&self.m.m[self.index]);
            self.index += 1;
            return result;
        }
        return None;
    }
}

fn get_neighbors(loc: &Location, m: &Map) -> Vec<Location> {
    let left = 0.max(loc.col-1);
    let right = (m.m[0].len()-1).min(loc.col+1);
    let top = 0.max(loc.row-1);
    let bottom = m.m.len().min(loc.row+1);

    let mut neighbors: Vec<Location> = Vec::new();
    for row in top..=bottom {
        for col in left..=right {
            if col != loc.col || row != loc.row {
                neighbors.push(Location::new(row, col));
            }
        }
    }
    return neighbors;
    }

#[derive(PartialEq, Eq, Hash)]
struct Number {
    left: usize,
    right: usize,
    row: usize,
    number: u32
}

fn turkey(loc: &Location, m: &Map) -> Number {
    if !m[loc].is_digit(10) {
        return Number{left: 0, right: 0, row: 0, number: 0};
    }
    let mut left = loc.col.clone();
    let mut right = loc.col.clone();
    while left > 0 && m[Location{row: loc.row, col: left-1}].is_digit(10) {
        left -= 1;
    }
    while right < m.m[0].len()-1 && m[Location{row: loc.row, col: right+1}].is_digit(10) {
        right += 1;
    }
    let number: u32 = m.m[loc.row][left..right+1].iter().collect::<String>().parse().unwrap();
    return Number{left: left, right: right, row: loc.row, number: number};
}

fn get_power(star: Location, m: &Map) -> u32 {
    let mut numbers:HashSet<Number> = HashSet::new();
    for loc in get_neighbors(&star, m) {
        if m[&loc].is_digit(10) {
            numbers.insert(turkey(&loc, m));
        }
    }
    if numbers.len() == 2 {
        return numbers.into_iter().fold(1, |acc, n| acc*n.number);
    }
    return 0;
}

fn main() {
    let path: &str = "input.txt";
    let result = fs::read_to_string(path);
    let contents = match result {
        Ok(result) => result,
        Err(_) => fs::read_to_string("day03/".to_string() + path).expect("File not found"),
    };

    let result: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let m: Map = Map{m: result};

    let mut stars: Vec<Location> = Vec::new();
    for (row, line) in m.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '*' {
                stars.push(Location::new(row, col));
            }
        }
    }

    let answer: u32 = stars.into_iter().map(|star| get_power(star, &m)).sum();
    println!("{}", answer);
}