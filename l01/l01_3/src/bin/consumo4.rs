fn main() {
    let s = String::from("hello"); 
    takes_ownership(&s);
    println!("{}", s); // stampa hello
}

fn takes_ownership(some_string: &str) { 
    let s = some_string.to_uppercase();
    println!("{}", s); // stampa HELLO
}
