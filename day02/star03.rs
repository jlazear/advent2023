use std::fs;

struct Bag {
    red: u32,
    green: u32,
    blue: u32
}

const BAG_MAX: Bag = Bag { red: 12, green:13, blue: 14};

fn parse_line(line: &str) -> bool {
    let s: Vec<&str> = line.split(' ').collect();
    for i in 1..s.len()/2 {
        let n: u32 = s[2*i].parse().unwrap();
        let color = s[2*i+1].trim_end_matches(|c| c == ',' || c == ';');
        let n_max = match color {
            "red" => BAG_MAX.red,
            "green" => BAG_MAX.green,
            "blue" => BAG_MAX.blue,
            &_ => 0
        };
        if n > n_max {
            return false;
        }
    }
    return true;
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum: u32 = 0;

    for (i, line) in contents.split('\n').enumerate() {
        if parse_line(line) {
            sum += (i as u32) + 1;
        }
    }

    println!("{sum}");
}