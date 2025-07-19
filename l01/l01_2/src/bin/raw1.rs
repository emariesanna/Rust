fn main() {
    let x = 10;
    let r: *const i32 = &x;

    unsafe {
        println!("Il valore di x tramite puntatore raw è: {}", *r);
    }
}