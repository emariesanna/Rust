use rand::{rng, Rng};
use std::io;

fn main() {
 println!("Inserisci una stringa:");
 let mut input = String::new();
 io::stdin()
     .read_line(&mut input)
     .expect("Errore nella lettura dell'input");
 let emoji_sequence = string_to_random_emojis(input);
 println!("Sequenza di emoji: {}", emoji_sequence);
}

fn string_to_random_emojis(input: &str) -> String {
let emojis = [
  "😀", "😃", "😄", "😁", "😆", "😅", "🤣", "😂", "🙂", "🙃",
  "😉", "😊", "😇", "🥰", "😍", "🤩", "😘", "😗", "😚", "😙",
  "🥲", "😋", "😛", "😜", "🤪", "😝", "🤑", "🤗", "🤭", "🤫",
  "🤔", "🤐", "🤨", "😐", "😑", "😶", "😏", "😒", "🙄", "😬",
  "🤥", "😌", "😔", "😪", "🤤", "😴", "😷", "🤒", "🤕", "🤢",
  "🤮", "🤧", "🥵", "🥶", "🥴", "😵", "🤯", "🤠", "🥳", "🥸",
  "😎", "🤓", "🧐", "😕", "😟", "🙁", "☹️", "😮", "😯", "😲",
  "😳", "🥺", "😦", "😧", "😨", "😰", "😥", "😢", "😭", "😱",
  "😖", "😣", "😞", "😓", "😩", "😫", "🥱", "😤", "😡", "😠",
  "🤬", "😈", "👿", "💀", "☠️", "💩", "🤡", "👹", "👺", "👻",
  "👽", "👾", "🤖", "😺", "😸", "😹", "😻", "😼", "😽", "🙀",
  "😿", "😾", "💋", "👋", "🤚", "🖐", "✋", "🖖", "👌", "🤌",
  "🤏", "✌️", "🤞", "🤟", "🤘", "🤙", "👈", "👉", "👆", "🖕",
  "👇", "☝️", "👍", "👎", "✊", "👊", "🤛", "🤜", "👏", "🙌",
  "👐", "🤲", "🤝", "🙏", "✍️", "💅", "🤳", "💪", "🦾", "🦿"
];
 let mut rng = rng();
 let mut result = String::new();
 // Per ogni carattere nella stringa di input, scegli un emoji casuale
 for _ in input.chars() {
  let random_index = rng.random_range(0..emojis.len());
  result.push_str(emojis[random_index]);
 }
 result
}
