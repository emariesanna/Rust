// Write a file

use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::
        create("output.txt")
        .expect(" File creation error")
        ;

    file
        .write_all("Hello!".as_bytes())
        .expect("unable to write to file");
}