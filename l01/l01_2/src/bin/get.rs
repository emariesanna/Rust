fn main() {
    let arr = [10, 20, 30];
    if let Some(val) = arr.get(3) {
        println!("Valore: {}", val); 
    }
    else {
        println!("Valore does not exist");
    }
}