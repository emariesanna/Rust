fn main() {
    let x = 5;

    // Creazione di puntatori nativi
    let ptr_immutabile: *const i32 = &x;
    let mut y = 10;
    let ptr_mutabile: *mut i32 = &mut y;

    println!("Indirizzi: {:?}, {:?}", ptr_immutabile, ptr_mutabile);
}