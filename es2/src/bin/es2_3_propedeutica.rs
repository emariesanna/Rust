use std::{fs, time::SystemTime};

fn read_write_file() {
    let args: Vec<String> = std::env::args().collect();
    let filename;
    if args.len() < 2 {
        filename = "test.txt";
    } else {
        filename = &args[1];
    }
    match fs::read_to_string(filename) {
        Ok(filestring) => {
            // let mut outputstring = String::new();
            // for _ in 0..10 {
            //     outputstring.push_str(&filestring);
            // }
            // if let Err(e) = fs::write(filename, &outputstring) {
            //     eprintln!("Failed to write to file: {}", e);
            // }
            for c in filestring.chars() {
                print!("{}  ", c);
            }
            println!();
            for b in filestring.bytes() {
                print!("{:02x} ", b);
            }
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
        }
    }
}

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn print_error(e: Error) {
        match e {
            Error::Simple(time) => println!("Simple error at {:?}", time),
            Error::Complex(time, msg) => println!("Complex error at {:?}: {}", time, msg),
        }
    }

fn mul(a: i32, b: i32) -> Result<u32, MulErr> {
    if a < 0 || b < 0 {
        return Err(MulErr::NegativeNumber);
    }
    match a.checked_mul(b) {
        Some(result) => Ok(result as u32),
        None => Err(MulErr::Overflow),
    }
}

pub enum MulErr {
    Overflow,
    NegativeNumber,
}

struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            size: 0,
            count: 0,
        }
    }

    pub fn size(self, size: u32) -> Node {
        Node {
            size,
            ..self
        }
    }

    pub fn count(self, count: u32) -> Node {
        Node {
            count,
            ..self
        }
    }

    pub fn to_string(&self) -> String {
        format!("name:{} size:{} count:{}", self.name, self.size, self.count)
    }

    pub fn grow(&mut self, size: u32) {
        self.size += size;
    }

    pub fn incr(&mut self) {
        self.count += 1;
    }
}


fn main(){
    let mut node: Node = Node::new("nodo")
        .size(100)
        .count(10);
    println!("{}", node.to_string());
    node.grow(50);
    println!("{}", node.to_string());
    node.incr();
    println!("{}", node.to_string());
}
