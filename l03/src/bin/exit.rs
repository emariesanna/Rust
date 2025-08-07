use std::fs::File;
use std::process;

fn main() {
    println!("Il programma sta per terminare con successo.");
    process::exit(100);
    // Qualsiasi codice dopo exit non verrà eseguito.
    println!("Questo non verrà stampato.");
}