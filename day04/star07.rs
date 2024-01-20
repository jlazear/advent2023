use std::{fs, io::BufRead, collections::HashSet};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

fn parse_line(line: String) -> u32 {
    let (winningstr, havestr) = line.split_once(':').unwrap().1
                                                .split_once('|').unwrap();
    let winning: HashSet<_> = winningstr.split_whitespace().collect();
    let have: HashSet<_> = havestr.split_whitespace().collect();

    let num: usize = winning.intersection(&have).collect::<HashSet<_>>().len();
    if num == 0 {
        return 0;
    }
    return 2u32.pow(u32::try_from(num-1).unwrap_or_else(|_| 0));
}

fn main() {
    let f = open_file("input.txt", "day04/");
    let reader = std::io::BufReader::new(f);
    let s: u32 = reader.lines()
                   .map(|line| parse_line(line.unwrap()))
                   .sum();
    println!("{}", s);
}