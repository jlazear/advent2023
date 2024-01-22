use std::{fs, io::{Read, self}};
use itertools::Itertools;

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SubRule {
    source: i64,
    source_end: i64,
    delta: i64
}

impl SubRule {
    fn make_subrule(subrule_str: &str) -> Self {
        let mut rule_list = subrule_str.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let destination = rule_list.next().unwrap();
        let source = rule_list.next().unwrap();
        let number = rule_list.next().unwrap();
        return Self {source, source_end: source+number-1, delta: destination-source};
    }
}

struct Rule {
    subrules: Vec<SubRule>
}

impl Rule {
    fn make_rule(rule_str: &str) -> Self {
        let mut subrules: Vec<SubRule> = Vec::new();
        for subrule_str in rule_str.split("\n").skip(1) {
            subrules.push(SubRule::make_subrule(subrule_str));
        }
        subrules.sort_unstable();
        return Rule {subrules};
    }

    fn split(&self, ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut newranges: Vec<(i64, i64)> = Vec::new();
        while ranges.len() > 0 {
            let (left, right) = ranges.pop().unwrap();
            let mut found: bool = false;
            for subrule in &self.subrules {
                let rule_left = subrule.source;
                let rule_right = subrule.source_end;
                let delta = subrule.delta;
                if rule_right < left {  // # 0
                    continue;
                } else if right < rule_left {  // # 1
                    found = true;
                    newranges.push((left, right));
                    break;
                } else if left < rule_left && right <= rule_right {  // # 2
                    found = true;
                    ranges.push((left, rule_left-1));
                    newranges.push((rule_left+delta, right+delta));
                    break;
                } else if rule_left <= left && rule_right < right {  // # 3
                    found = true;
                    ranges.push((rule_right+1, right));
                    newranges.push((left+delta, rule_right+delta));
                    break;
                } else if rule_left <= left && right <= rule_right { // # 4
                    found = true;
                    newranges.push((left+delta, right+delta));
                    break;
                } else if left < rule_left && rule_right < right {  // # 5
                    found = true;
                    ranges.push((rule_right+1, right));
                    ranges.push((left, rule_left-1));
                    newranges.push((rule_left+delta, rule_right+delta));
                    break;
                }
            }
            if !found {
                newranges.push((left, right));
            }
        }
        newranges.sort_unstable();
        return newranges;
    }
}

fn main() -> io::Result<()> {
    let mut f = open_file("input.txt", "day05/");

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let mut ssplit = s.split("\n\n");
    let firstline: &str = ssplit.next().unwrap();

    let seeds: Vec<i64> = firstline.split_whitespace().skip(1)
                               .map(|x| x.parse::<i64>().unwrap()).collect();

    let rules: Vec<Rule> = ssplit.map(|x| Rule::make_rule(x)).collect();

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for (start, length) in seeds.into_iter().tuples() {
        ranges.push((start, start+length));
    }
    ranges.sort_unstable();
    ranges.reverse();

    for rule in rules {
        ranges = rule.split(&mut ranges);
    }

    println!("{}", ranges.iter().min().unwrap().0);
    return Ok(());
}