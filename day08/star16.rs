use std::{error::Error, fs, io::{BufReader, BufRead}, collections::HashMap, rc::{Rc, Weak}, cell::RefCell};

fn open_file(path: &str, dir: &str) -> std::fs::File {
    return match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => fs::File::open(dir.to_string() + path).expect("File not found!"),
    };
}

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Weak<RefCell<Node>>>,
    right: Option<Weak<RefCell<Node>>>
}

impl Node {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(
            Node { name: String::from(name),
                   left: None,
                   right: None}
        ));
    }

    fn set_left(&mut self, left: Rc<RefCell<Node>>) {
        self.left = Some(Rc::downgrade(&left));
    }

    fn set_right(&mut self, right: Rc<RefCell<Node>>) {
        self.right = Some(Rc::downgrade(&right));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = BufReader::new(open_file("input.txt", "day08/"));

    let mut iter = buf.lines();
    let instructions: Vec<char> = iter.next().unwrap()?.chars().collect();
    iter.next();

    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    for line in iter {
        let line = line?;
        let mut lineiter = line.split(" = ");
        let name = lineiter.next().unwrap().trim();
        let rhs = lineiter.next().unwrap().trim().trim_matches(|c| c == '(' || c == ')');
        let mut rhsiter = rhs.split(',');
        let left = rhsiter.next().unwrap().trim();
        let right = rhsiter.next().unwrap().trim();

        let node = match nodes.get(name) {
            None => {Node::new(name)}  // make a new node if it doesn't exist
            Some(node) => {Rc::clone(node)}
        };
        match nodes.get(left) {
            None => {  // make a new node if it doesn't exist
                let leftnode = Node::new(left);
                nodes.insert(left.to_string(), Rc::clone(&leftnode));
                node.borrow_mut().set_left(Rc::clone(&leftnode));
            }
            Some(leftnode) => {
                node.borrow_mut().set_left(Rc::clone(&leftnode));
            }
        }
        match nodes.get(right) {
            None => {  // make a new node if it doesn't exist
                let rightnode = Node::new(right);
                nodes.insert(right.to_string(), Rc::clone(&rightnode));
                node.borrow_mut().set_right(Rc::clone(&rightnode));
            }
            Some(rightnode) => {
                node.borrow_mut().set_right(Rc::clone(&rightnode));
            }
        }
        nodes.insert(name.to_string(), Rc::clone(&node));
    }

    let mut current_node = nodes.get("AAA").unwrap().clone();

    let mut n: usize = 0;
    let mut name = current_node.borrow().name.clone();
    while name != "ZZZ" {
        let instruction = instructions[n % instructions.len()];
    
        current_node = if instruction == 'L' {
            current_node.borrow().left.as_ref().unwrap().upgrade().unwrap()
        } else {
            current_node.borrow().right.as_ref().unwrap().upgrade().unwrap()
        };
        name = current_node.borrow().name.clone();
        n += 1;
    }
    println!("{n}");
    return Ok(());
}