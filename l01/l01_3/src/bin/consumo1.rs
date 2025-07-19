fn main() {
    let s = String::from("hello"); 
    takes_ownership(s);
}

fn takes_ownership(some_string: String) { 
    let s = some_string.to_uppercase();
    println!("{}", s);
}
