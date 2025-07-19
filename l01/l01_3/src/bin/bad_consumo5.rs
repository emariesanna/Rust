fn main() {
    let mut s = String::from("hello");
    let p = &s;

    s = takes_ownership(&s);
    println!("{}", p);
    println!("{}", s); // stampa HELLO
}

fn takes_ownership(some_string: &str) -> String {
    let s = some_string.to_uppercase();
    println!("{}", s); // stampa HELLO
    s
}