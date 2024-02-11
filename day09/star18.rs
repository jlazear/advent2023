use std::{error::Error, fs, io::{BufReader, BufRead}};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

#[allow(non_snake_case)]
fn delta(N: i64) -> Vec<i64> {
    if N == 0 {
        return vec![1_i64];
    }
    let mut prevdelta = delta(N-1);
    prevdelta.push(0);
    for i in (0..N).rev() {
        prevdelta[(i+1) as usize] -= prevdelta[i as usize];
    }
    return prevdelta;
}

#[allow(non_snake_case)]
fn next_value(xs: &Vec<i64>, N: i64) -> i64 {
    let d = delta(N);
    let next_value = (0..(d.len()-1))
        .map(|i| -d[d.len()-1-i] * xs[xs.len()+1-d.len()+i])
        .sum();
    return next_value;
}

#[allow(non_snake_case)]
fn prev_value(xs: &Vec<i64>, N: i64) -> i64 {
    let d = delta(N);
    let mut next_value = (0..(d.len()-1))
        .map(|i| -d[d.len()-2-i] * xs[i])
        .sum();
    next_value *=  *d.last().unwrap();
    return next_value;
}

#[allow(non_snake_case)]
fn check_order(xs: &Vec<i64>, N:i64) -> bool {
    let mut xs0 = xs[0..(N as usize)].to_vec();
    while xs0.len() < xs.len() {
        let v = next_value(&xs0, N);
        xs0.push(v);
        if xs0.last().unwrap() != &xs[(xs0.len()-1) as usize] {
            return false;
        }
    }
    return true;
}

#[allow(non_snake_case)]
fn solve(xs: Vec<i64>) -> i64 {
    let mut N = 1;
    while !check_order(&xs, N) {
        N += 1;
    }
    let solution = prev_value(&xs, N);
    return solution;
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(open_file("input.txt", "day09/"));

    let s: i64 = buf.lines()
        .map( |line| line.unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>())
        .map( |v| solve(v))
        .sum();

    println!("{s}");
    return Ok(());
}