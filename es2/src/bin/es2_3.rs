use std::fs;

pub enum Error {
    TooFewArguments(u8),
    CannotWriteFile,
    InvalidArgument(String),
}

fn parse_numeri(arguments: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    arguments
        .split(',')
        .map(|s| s.trim().parse::<u32>())
        .collect()
}


fn nuova_board(arguments: &str){
    let numeri: Result<_, std::num::ParseIntError> = arguments
        .split(',')
        .map(|s| s.trim().parse::<u32>())
        .collect();
    match numeri {
        Ok(numeri) => {
            if numeri.len() == 4 && numeri.iter().all(|&n| n >= 1 && n <= 20){
                valori
            } else {
                Error::InvalidArgument("Invalid numbers")
            }
            
        },
        Err(numeri){
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