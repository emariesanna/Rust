#[deny(unused_variables)]
#[forbid(unsafe_code)]

fn main() {
    #[allow(unused_variables)]
    let x = 5; // Questo causerà un errore di compilazione

    let y=5;
    println!("Hello, world!");
}