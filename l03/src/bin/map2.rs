fn main() {
    fn parse_and_add_one(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map(|n| n + 1) // map() viene chiamato solo se parse() restituisce Ok
            .map_err(|_| format!("Impossibile convertire '{}' in un intero", s))
    }

    let success = parse_and_add_one("10");
    println!("Successo: {:?}", success); // Output: Successo: Ok(11)

    let failure = parse_and_add_one("abc");
    println!("Fallimento: {:?}", failure); // Output: Fallimento: Err("Impossibile convertire 'abc' in un intero")
}