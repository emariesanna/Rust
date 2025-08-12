use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

pub enum Error {
    CannotOpenFile,
    CannotWriteFile,
    ParseError,
    DirectionError,
    IncorrectArgumentsNumber,
    IncorrectArgumentsValue
}

impl Error {
    pub fn print_error(&self) {
        eprintln!("{}", 
            match self {
                Self::CannotOpenFile => "Cannot open file.",
                Self::CannotWriteFile => "Cannot write file.",
                Self::ParseError => "Cannot parse arguments.",
                Self::DirectionError => "Invalid boat direction."
                Self::IncorrectArgumentsNumber => "Incorrect arguments number.",
                Self::IncorrectArgumentsValue => "Incorrect arguments values.",
            }
        );
    }
}

fn nuova_board(filename: &str, arguments: &str) -> Result<(), Error>{
    let numeri = arguments
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| Error::ParseError)?;
        
    if numeri.len() != 4 {
        Err(Error::IncorrectArgumentsNumber)?
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .map_err(|_| Error::CannotOpenFile)?;

    writeln!(file, "{}", arguments).map_err(|_| Error::CannotWriteFile)?;
    for _ in 0..20 {
        let mut line = String::new();
        for _ in 0..20 {
            line.push(' ');
        } 
        writeln!(file, "{}", line).map_err(|_| Error::CannotWriteFile)?;
    }
    Ok(())
}

fn aggiungi_nave(filename: &str, arguments: &str) -> Result<(), Error>{
    let board = read_to_string(filename).map_err(|_| Error::CannotOpenFile)?;
    let mut rows: Vec<String> = board.lines().skip(1).map(|s| s.to_string()).collect();

    let argomenti: Vec<String> = arguments.split(",").map(|s| s.to_string()).collect();
    let direzione = &argomenti[0];
    if direzione != "V" || direzione != "H" {
        Err(Error::DirectionError)?
    }
    let valori: Vec<u8> = argomenti[1..4]
        .iter()
        .map(|s |s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| Error::ParseError)?;

    let lunghezza = valori[0];
    let mut x = valori[1];
    let mut y = valori[2];

    for _ in 0..lunghezza {
        let row = &mut rows[(y - 1) as usize];
        let mut chars: Vec<char> = row.chars().collect();
        chars[(x - 1) as usize] = 'B';
        *row = chars.into_iter().collect();
        if direzione == "V" {
            y -= 1;
        } else if direzione == "H" {
            x -= 1;
        }
    }

    Ok(())

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
        "new" => {
            if let Err(err) = nuova_board(filename, arguments) {
                err.print_error();
                return;
            }
        }
        "add_boat" => { 
            if let Err(err) = aggiungi_nave(filename, arguments) {
                err.print_error();
                return;
            }
        }
        &_ => { 
            println!("Invalid arguments.");
            return
        }
    }
}