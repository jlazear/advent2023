use std::fs;
use std::ops::Index;

#[derive(Debug)]
struct Neighbors {
    number: u32,
    neighbors: Vec<Location>
}

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

fn get_neighbors(locs: &Vec<Location>, m: &Map) -> Vec<Location> {
    let xs: Vec<&usize> = locs.iter().map(|loc| &loc.col).collect();
    let xmin = **xs.iter().min().unwrap();
    let xmax = **xs.iter().max().unwrap();
    let y = locs[0].row;
    let mylen = m.m.len();
    let mxlen = m.m[0].len();

    let mut neighbors: Vec<Location> = Vec::new();
    if xmin >= 1 {
        neighbors.push(Location::new(y, xmin-1));
    }
    if xmax+1 < mxlen {
        neighbors.push(Location::new(y, xmax+1));
    }
    if y+1 < mylen {
        let left = if xmin > 1 {xmin-1} else {0};
        let right = if mxlen < xmax+2 {mxlen} else {xmax+2};
        neighbors.extend((left..right).map(|x| Location::new(y+1, x)));
    }
    if y >= 1 {
        let left = if xmin > 1 {xmin-1} else {0};
        let right = if mxlen < xmax+2 {mxlen} else {xmax+2};
        neighbors.extend((left..right).map(|x| Location::new(y-1, x)));
    }
    return neighbors;
}

fn parse_line(line: &Vec<char>, row: usize, m: &Map, numbers: &mut Vec<Neighbors>) {
    let mut n: u32 = 0;
    let mut locs: Vec<Location> = Vec::new();
    for (col, c) in line.iter().enumerate() {
        if c.is_digit(10) {
            n = 10*n + c.to_digit(10).unwrap();
            locs.push(Location::new(row, col));
        } else {
            if n != 0 {
                numbers.push(Neighbors{number: n, neighbors: get_neighbors(&locs, &m)});
                n = 0;
                locs.clear();
            }
        }
    }
    if n != 0 {
        numbers.push(Neighbors{number: n, neighbors: get_neighbors(&locs, &m)});
    }
}

fn is_valid(neighbors : &Neighbors, m: &Map) -> bool {
    for loc in &neighbors.neighbors {
        let c: char = m[loc];
        if !c.is_digit(10) && c != '.' {
            return true;
        }
    }
    return false;
}

fn print_neighbors(neighbors: &Neighbors, m: &Map) -> String {
    let ys: Vec<usize> = neighbors.neighbors.iter().map(|loc| loc.row).collect();
    let xs: Vec<usize> = neighbors.neighbors.iter().map(|loc| loc.col).collect();
    let xmin = *xs.iter().min().unwrap();
    let xmax = *xs.iter().max().unwrap();
    let ymin = *ys.iter().min().unwrap();
    let ymax = *ys.iter().max().unwrap();

    // let mut s: String = String::new();
    let mut sx: Vec<String> = Vec::new();
    for y in ymin..=ymax {
        let mut sy: Vec<char> = Vec::new();
        for x in xmin..=xmax {
            sy.push(m[Location{row: y, col: x}]);
        }
        sx.push(sy.into_iter().collect());
    }
    let s = sx.join("\n");
    println!("{}", s);
    return s;
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

    let mut numbers: Vec<Neighbors> = Vec::new();
    for (row, line) in m.iter().enumerate() {
        parse_line(line, row, &m, &mut numbers);
    }
    let mut sum: u32 = 0;
    for neighbors in numbers {
        if is_valid(&neighbors, &m) {
            sum += &neighbors.number;
        }
    }
    println!("{}", sum);
}