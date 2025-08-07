fn main() {
    // Un Option che contiene un valore (Some)
    let valore_presente: Option<i32> = Some(10);

    // Usiamo unwrap() per ottenere il valore.
    let numero = valore_presente.unwrap();
    println!("Il numero è: {}", numero); // Output: Il numero è: 10

    // Un Option che non contiene un valore (None)
    let valore_assente: Option<String> = None;

    // Se chiamiamo unwrap() su un Option che è None, il programma andrà in panic.
    // Il seguente codice causerà un crash:
    // let testo = valore_assente.unwrap();
    // println!("Il testo è: {}", testo);

    // Per evitare il panic, dovremmo sempre verificare se l'Option è Some prima di chiamare unwrap()
    if valore_assente.is_some() {
        let testo = valore_assente.unwrap();
        println!("Il testo è: {}", testo);
    } else {
        println!("Il valore di testo è assente."); // Output: Il valore di testo è assente.
    }
}
