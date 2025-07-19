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
    let mut p1 = Point { x: 1, y: 2 };
    println!("creo p1 {:?}", p1);
    {
        p1 = Point { x: 10, y: 2 };
        println!("Ho creato nuovo p1 {:?}: c'e' stato un MOVIMENTO", p1);
        println!("esco dal blocco interno");
    }
}