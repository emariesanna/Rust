fn main() {
    'outer: loop {
        println!("Entrato nel ciclo esterno");

        'inner: loop {
            println!("Entrato nel ciclo interno");

            // La prossima istruzione interromperebbe il ciclo interno
            //break;

            // Così si interrompe il ciclo esterno
            break 'outer;
        }
        //Il programma non raggiunge mai questa posizione
    }
    println!("Terminato il ciclo esterno");
}