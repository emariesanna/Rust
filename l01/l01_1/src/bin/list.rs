// List directory contents

use std::fs;

fn main() {
    let entries = fs::read_dir("./")
        .expect("Failed to read dir");

    for entry in entries {
        let path = entry.expect("Failed to read")
            .path();

        println!("{}", path.display());
    }
}