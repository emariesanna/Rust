use std::fs;

enum Error {
    TooFewArguments(u8),
    CannotWriteFile,
    InvalidArgument(String),
}

fn new_board(filename: &str, arguments: &str) -> Result<(), Error> {
    let numeri: Result<Vec<u32>, _> = arguments
        .split(',')
        .map(|s| s.trim().parse::<u32>())
        .collect();

    let valori = match numeri {
        Ok(valori) if valori.len() == 4 && valori.iter().all(|&n| n >= 1 && n <= 20) => {
            println!("Stringa valida: {:?}", valori);
            valori
        }
        Ok(_) | Err(_) => {
            return Err(Error::InvalidArgument(arguments.to_string()));
        }
    };
    fs::write(filename, arguments).map_err(|_| Error::CannotWriteFile)
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Missing arguments.");
        return
    }
    let filename = &args[1];
    let command = &args[2];
    let arguments = &args[3];
    
    match command.as_str() {
        "new" => { new_board(filename, arguments); }
        "add_boat" => { 
            let file = fs::read(filename);
        }
        &_ => { 
            println!("Invalid arguments.");
            return
        }
    }
}