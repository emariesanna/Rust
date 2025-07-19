fn make_box(a: i32) -> Box<(i32,i32)> {
    let r = Box::new( (a, 1) );
    return r;
}

fn main() {
    let mut b = make_box(5);

    b.0 = b.0 * 2;

    println!("{:#?}", b);
}
