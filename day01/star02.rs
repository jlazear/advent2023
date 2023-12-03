use std::fs;

fn parse_line(line: &str) -> u32 {
    let mut digits = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap());
        } else if line[i..].starts_with("one") {
            digits.push(1);
        } else if line[i..].starts_with("two") {
            digits.push(2);
        } else if line[i..].starts_with("three") {
            digits.push(3);
        } else if line[i..].starts_with("four") {
            digits.push(4);
        } else if line[i..].starts_with("five") {
            digits.push(5);
        } else if line[i..].starts_with("six") {
            digits.push(6);
        } else if line[i..].starts_with("seven") {
            digits.push(7);
        } else if line[i..].starts_with("eight") {
            digits.push(8);
        } else if line[i..].starts_with("nine") {
            digits.push(9);
        }
    }
    return 10*digits[0] + digits[digits.len()-1];
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