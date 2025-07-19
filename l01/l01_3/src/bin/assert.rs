fn somma(a: i32, b: i32) -> i32 {
a + b
}

fn main() {
    // Test della funzione somma
    let risultato = somma(2, 3);

    // Verifica che il risultato sia 5
    assert_eq!(risultato, 5);
    println!("Il test è passato!");

    // Questo test passerà

    // Proviamo con un test che fallisce
    let nuovo_risultato = somma(2, 2);

    // Verifica che il risultato sia 5 (che è sbagliato)
    assert_eq!(nuovo_risultato, 5);

    // Questa riga non verrà mai eseguita perché il programma
    // si interrompe all'assert fallito
    println!("Questo non verrà stampato");
}

