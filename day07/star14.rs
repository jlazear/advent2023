use std::{io::{BufReader, BufRead}, fs, error::Error, collections::HashMap};
use std::hash::Hash;

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

fn maketrans(from: &str, to: &str) -> HashMap<char, char> {
    let mut trans: HashMap<char, char> = HashMap::new();

    for (cf, ct) in from.chars().zip(to.chars()) {
        trans.insert(cf, ct);
    }
    return trans;
}

trait StringTranslate {
    fn translate(&self, trans: &HashMap<char, char>) -> String;
}

impl StringTranslate for String {
    fn translate(&self, trans: &HashMap<char, char>) -> String {
        return self.chars()
           .filter_map(|c| trans.get(&c).cloned())
           .collect();
    }
}

fn counter<T, I>(x: I) -> HashMap<T, i32> 
where T: Eq + Hash,
      I: Iterator<Item=T>,
      {
        let mut c: HashMap<T, i32> = HashMap::new();
        for item in x {
            *c.entry(item).or_insert(0) += 1;
        }
        return c;
}

fn most_common<T, U>(m: &HashMap<T, U>) -> Option<&T>
where U: Ord {
    return m.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k);
}

fn main() -> Result<(), Box<dyn Error>>  {
    let f = open_file("input.txt", "day07/");
    let buf = BufReader::new(f);

    let trans = maketrans("J23456789TQKA", "23456789ABCDE");

    let mut data: Vec<(i32, i32, String, i32)> = Vec::new();
    for line in buf.lines() {
        if let Ok(ref s) = line {
            let mut iter = s.split_whitespace();
            let hand = String::from(iter.next().unwrap());
            let modhand = hand.translate(&trans);
            let bid = iter.next().unwrap().parse::<i32>()?;
            let mut c = counter(hand.chars());
            if c.contains_key(&'J') && c.len() > 1 {
                let cj = c.remove(&'J').unwrap();
                let maxc = most_common(&c).unwrap().clone();  // how to do this without cloning?
                if let Some(cmaxc) = c.get_mut(&maxc) {
                    *cmaxc += cj;
                }
            }
            let cardinality = c.len() as i32;
            let biggest = c.values().max().unwrap().clone();
            data.push((-cardinality, biggest, modhand, bid));
        }
    }

    data.sort();
    let sum: i32 = data.into_iter().enumerate()
        .map(|(i, (_, _, _, bid))| ((i as i32)+1)*bid)
        .sum();
    println!("{sum}");
    return Ok(());
}