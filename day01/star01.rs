use std::fs;

fn parse_line(line: &str) -> u32 {
    let mut digits = Vec::new();
    for c in line.chars() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10));
        }
    }
    return 10*digits[0].unwrap() + digits[digits.len()-1].unwrap();
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");

    let mut sum: u32 = 0;
    for (i, line) in contents.split('\n').enumerate() {
        sum += parse_line(line);
    }
    println!("{sum}");
}