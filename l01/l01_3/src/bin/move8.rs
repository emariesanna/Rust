fn main() {

    let read_string = "Dante Alighieri".to_string();

    println!("{read_string}");

    let mut second_string = read_string;          // Movimento su una variabile mutabile

    second_string = second_string.to_uppercase(); 

    println!("{second_string}");

}