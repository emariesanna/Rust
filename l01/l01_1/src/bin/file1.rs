// Read a file line by line
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("hello.txt")
        .expect("File error")
        ;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Error reading file"))
    }
}