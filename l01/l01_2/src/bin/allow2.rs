// Disabilita il warning per nomi non snake_case su una funzione
#[allow(non_snake_case)]
fn StranaFunzioneConMaiuscole() {
    println!("Funzione con nome non convenzionale!");
}

// Disabilita lo stesso warning su una variabile
fn main() {
    #[allow(non_snake_case)]
    let MiaVariabileConMaiuscole = 42; // Variabile con nome non snake_case
    println!("MiaVariabile con nome {}", MiaVariabileConMaiuscole);

    let risultato = calcolaQualcosa(10, 20);
    println!("Risultato: {}", risultato);

    StranaFunzioneConMaiuscole()
}

// Anche sui parametri delle funzioni
#[allow(non_snake_case)]
fn calcolaQualcosa(ValoreA: i32, ValoreB: i32) -> i32 {
    ValoreA + ValoreB
}