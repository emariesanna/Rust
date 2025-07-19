fn main() {
    let x = -1;
    let ptr = &x;
    println!("x: {}", x);
    println!("ptr: {:x}", ptr);
    println!("indirizzo: {:p}", ptr);
}