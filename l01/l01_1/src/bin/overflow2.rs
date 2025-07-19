fn main() {
    let mut i = 2_000_000_000;
    let j = i;

    i = i + j;

    println!("{}", i);
}