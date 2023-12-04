use std::fs;
use std::cmp::max;

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32
}

impl Bag {
    fn power(&self) -> u32 {
        return self.red*self.green*self.blue;
    }
}

fn parse_line(line: &str) -> u32 {
    let mut bag: Bag = Bag { red: 0, green:0, blue: 0};
    let s: Vec<&str> = line.split(' ').collect();
    for i in 1..s.len()/2 {
        let n: u32 = s[2*i].parse().unwrap();
        let color = s[2*i+1].trim_end_matches(|c| c == ',' || c == ';');
        match color {
            "red" => bag.red = max(n, bag.red),
            "green" => bag.green = max(n, bag.green),
            "blue" => bag.blue = max(n, bag.blue),
            &_ => ()
        }
    }
    return bag.power();
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum: u32 = 0;

    for line in contents.split('\n') {
        sum += parse_line(line);
    }

    println!("{sum}");
}