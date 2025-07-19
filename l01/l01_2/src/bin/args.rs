use std::env::args;
fn main() {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() > 0 { // we have args!
        for i in 0..args.len() {
            println!("{}", args[i]);
        }
    }
}