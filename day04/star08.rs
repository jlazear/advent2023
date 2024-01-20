use std::{fs, io::BufRead, collections::{HashSet, HashMap}};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

fn parse_line(line: String) -> usize {
    let (winningstr, havestr) = line.split_once(':').unwrap().1
                                                .split_once('|').unwrap();
    let winning: HashSet<_> = winningstr.split_whitespace().collect();
    let have: HashSet<_> = havestr.split_whitespace().collect();

    let num: usize = winning.intersection(&have).collect::<HashSet<_>>().len();
    return num;
}

fn main() {
    let f = open_file("input.txt", "day04/");
    let reader = std::io::BufReader::new(f);
    let cards: Vec<usize> = reader.lines()
                   .map(|line| parse_line(line.unwrap()))
                   .collect();

    let mut c: HashMap<_, u32> = (0..cards.len()).map(|key| (key, 1)).collect();
    for i in 0..(c.len()) {
        let ci = *c.get(&i).unwrap();
        for j in 0..(cards[i]) {
            *c.entry(i+j+1).or_insert(1) += ci;
        }
    }
    println!("{}", c.into_values().sum::<u32>());
}