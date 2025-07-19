// Esempio 1: Disabilita il warning "dead_code" per una funzione
#[allow(dead_code)]
fn funzione_inutilizzata() {
    println!("Questa funzione non viene mai chiamata");
}

// Esempio 2: Disabilita il warning "unused_variables" in un blocco
fn main() {
    #[allow(unused_variables)]
    let x = 10; // Variabile non utilizzata, ma il warning è soppresso

    // Esempio 3: Disabilita più warning contemporaneamente
    #[allow(unused_mut, unused_assignments, unused_variables)]
    {
        let mut y = 5;
        y = 10; // Assegnazione non necessaria (ma warning soppressi)
    }

    println!("Funziona!");
}