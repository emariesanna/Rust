use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

pub enum Error {
    CannotOpenFile,
    CannotWriteFile,
    ParseError,
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
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::ParseError)?;
        
    if numeri.len() != 4 {
        Err(Error::IncorrectArgumentsNumber)?
    }

    let mut file = OpenOptions::new()
        .create(true)      // crea il file se non esiste
        .append(true)      // aggiunge alla fine del file
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
    let mut rows: Vec<String> = board.lines().map(|s| s.to_string()).collect();
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