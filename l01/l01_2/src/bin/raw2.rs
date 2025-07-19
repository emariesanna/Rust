fn main() {
    let mut x = 10;

    // Otteniamo un puntatore mutabile a x
    let r: *mut i32 = &mut x;

    // Accediamo e modifichiamo il valore di x attraverso il puntatore raw

    unsafe {
        // Modifichiamo il valore di x tramite il puntatore mutabile
        *r = 20;
    }

    // Ora x è stato cambiato
    println!("Il nuovo valore di x è: {}", x); // Stampa: Il nuovo valore di x è: 20
}
