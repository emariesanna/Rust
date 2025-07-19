fn main() {

    let a =10;
    let mut b = 2;
    b = dec(b,b);

    let c = a/b;
    println!("{}", c);
}

fn dec(a: i32, b: i32) -> i32 {
    a-b
}