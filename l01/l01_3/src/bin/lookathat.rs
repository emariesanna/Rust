#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Drop for Point {
    fn drop(&mut self) {
        println!("Distruggo {:?}", self);
    }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };
    println!("creo p1 {:?}", p1);
    {
        let p1 = Point { x: 10, y: 2 };
        println!("Nuovo p1 {:?}:", p1);
        println!("esco dal blocco interno");
    }
    println!("leggo p1 {:?}", p1);
}