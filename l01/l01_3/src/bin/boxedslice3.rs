fn main() {
    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);

    // Ottenere una slice da un Box<[i32]>
    let slice: &[i32] = &*boxed_slice;

    println!("{:?}", slice); // Stampa: [1, 2, 3, 4, 5]

    // Creare una slice che considera solo una parte del vettore contenuto nel Box

    let slice2: &[i32] = &boxed_slice[1..3]; // Da 2 a 3 (escluso 3)

    println!("{:?}", slice2); // Stampa: [2, 3]

}

