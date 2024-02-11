use std::{collections::HashSet, error::Error, fs, io::{BufRead, BufReader}, mem};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

type Coord = (usize, usize);

fn nearest_above<T> (x0: &T, xs: &[T]) -> usize
where for<'a> &'a T: PartialOrd<&'a T> 
 {
    for (i, x) in xs.iter().enumerate() {
        if x > x0 {
            return i;
        }
    }
    return xs.len();
}

fn nearest_below<T> (x0: &T, xs: &[T]) -> Option<usize>
where T: std::cmp::PartialOrd
 {
    for i in 0..xs.len() {
        let n = xs.len() - 1 - i;
        if &xs[n] < x0 {
            return Some(n);
        }
    }
    return None;
}

#[allow(non_snake_case)]
fn expanded_L1_norm(galaxy0: &Coord, galaxy1: &Coord, emptyrows: &[usize], emptycols: &[usize]) -> i64 {
    let (mut x0, mut y0) = galaxy0;
    let (mut x1, mut y1) = galaxy1;
    if x1 < x0 {
        mem::swap(&mut x0, &mut x1);
    }
    if y1 < y0 {
        mem::swap(&mut y0, &mut y1);
    }
    let nx = nearest_below(&x1, &emptyrows).map(|x| x as i64).unwrap_or(-1) - (nearest_above(&x0, &emptyrows) as i64) + 1;
    let ny = nearest_below(&y1, &emptycols).map(|x| x as i64).unwrap_or(-1) - (nearest_above(&y0, &emptycols) as i64) + 1;
    let norm = (x1 as i64 - x0 as i64 + nx) + (y1 as i64 - y0 as i64 + ny);
    return norm;
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(open_file("input.txt", "day11/"));

    let mut galaxies: Vec<Coord> = Vec::new();
    let mut rows: HashSet<usize> = HashSet::new();
    let mut cols: HashSet<usize> = HashSet::new();

    let mut maxrow: usize = 0;
    let mut maxcol: usize = 0;
    for (row, line) in buf.lines().enumerate() {
        maxrow = row;
        for (col, c) in line?.chars().enumerate() {
            maxcol = col;
            if c == '#' {
                galaxies.push((row, col));
                rows.insert(row);
                cols.insert(col);
            }
        }
    }

    let mut emptyrows: Vec<usize> = HashSet::from_iter((0..maxrow+1).into_iter()).difference(&rows).map(|x| *x).collect();
    let mut emptycols: Vec<usize> = HashSet::from_iter((0..maxcol+1).into_iter()).difference(&cols).map(|x| *x).collect();
    emptyrows.sort_unstable();
    emptycols.sort_unstable();
    let emptyrows = emptyrows;
    let emptycols = emptycols;

    let mut s: i64 = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in &galaxies[i+1..] {
            let distance = expanded_L1_norm(galaxy1, galaxy2, &emptyrows, &emptycols);
            s += distance;
        }
    }

    println!("{s}");

    return Ok(());
}