#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let mut p2 = p1;

    // p1 è stato consumato e non è più accessibile
    // println!("Punto 1: {:?}", p1);

    println!("Punto 2: {:?}", p2); // Output: Punto 2: Punto { x: 1, y: 2 }

    p2 = Punto { x: 3, y: 4 };
    println!("Punto 2 (modificato): {:?}", p2); // Output: Punto 2 (non modificato): Punto { x: 1, y: 2 }
}