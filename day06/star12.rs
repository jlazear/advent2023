use std::{io::{BufReader, BufRead, self}, fs};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

fn solutions(t: f64, mut d: f64) -> (f64, f64) {
    d += 1.0e-6; 
    let delta = f64::sqrt(t*t - 4.0*d);
    let plus = (t + delta)/2.0;
    let minus = (t - delta)/2.0;
    return (plus, minus);
}

fn n_solutions(plus: f64, minus: f64) -> i64 {
    let left = minus.ceil() as i64; 
    let right = plus.floor() as i64;
    return right - left + 1;
}
fn main() -> io::Result<()> {
    let f = open_file("input.txt", "day06/");
    let buf = BufReader::new(f);

    let mut line_iter = buf.lines();
    let time: f64 = line_iter.next().unwrap()?.split_whitespace().skip(1)
            .fold(String::new(), |s, a| s + a).parse().unwrap();
    let distance: f64 = line_iter.next().unwrap()?.split_whitespace().skip(1)
            .fold(String::new(), |s, a| s + a).parse().unwrap();

    let (plus, minus) = solutions(time, distance);
    let n = n_solutions(plus, minus);

    println!("{}", n);

    return Ok(());
}