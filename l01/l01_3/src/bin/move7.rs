fn genera_vec_zero(dimensione: usize) -> Vec<i32> {
    // Crea un nuovo Vec con la capacità specificata.
    // Questo è più efficiente perché evita riallocazioni multiple
    // che potrebbero avvenire con successivi push.
    let mut vec = Vec::with_capacity(dimensione);

    // Aggiunge 'dimensione' numero di zeri al Vec.
    for _ in 0..dimensione {
        vec.push(0);
    }

    // Restituisce il Vec riempito di zeri.
    vec
}

fn main() {
    let dimensione_desiderata = 5;
    let vettore_di_zeri = genera_vec_zero(dimensione_desiderata);
    println!("Vettore di zeri: {:?}", vettore_di_zeri); // Output: Vettore di zeri: [0, 0, 0, 0, 0]
}