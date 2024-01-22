use std::{fs, io::{Read, self}};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

struct SubRule {
    source: i64,
    source_end: i64,
    destination: i64
}

impl SubRule {
    fn make_subrule(subrule_str: &str) -> Self {
        let mut rule_list = subrule_str.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let destination = rule_list.next().unwrap();
        let source = rule_list.next().unwrap();
        let number = rule_list.next().unwrap();
        return Self {source, source_end: source+number, destination};
    }

    fn apply(&self, i: i64) -> i64 {
        return &self.destination + (i - &self.source);
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
        return Rule {subrules};
    }
}
impl Rule {
    fn dispatch(&self, x: i64) -> i64 {
        for subrule in &self.subrules {
            if subrule.source <= x && x <= subrule.source_end {
                return subrule.apply(x);
            }
        }
        return x;
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

    let mut locations: Vec<i64> = Vec::new();

    for mut seed in seeds {
        for rule in &rules {
            seed = rule.dispatch(seed);
        }
        locations.push(seed);
    }
    println!("{}", locations.iter().min().unwrap());
    return Ok(());
}