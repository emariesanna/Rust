fn main() {
    let x = -1;

    if x < 0 {
        panic!("Valore negativo non supportato: {}", x);
    }

    println!("Il valore di x è: {}", x);
}