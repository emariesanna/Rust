#[derive(Debug, Clone, Copy)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let p2 = p1;

    println!("Punto 1: {:?}", p1); // Output: Punto 1: Punto { x: 1, y: 2 }
    println!("Punto 2: {:?}", p2); // Output: Punto 2: Punto { x: 1, y: 2 }

    let mut p1 = Punto { x: 3, y: 4 };
    println!("Punto 1 modificato: {:?}", p1); // Output: Punto 1 modificato: Punto { x: 3, y: 4 }
    println!("Punto 2 (non modificato): {:?}", p2); // Output: Punto 2 (non modificato): Punto { x: 1, y: 2 }
}
