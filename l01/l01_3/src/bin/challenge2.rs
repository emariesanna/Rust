use rand::Rng;
fn main() {

    let mut b = Box::new(84);
    let r = & mut b;

    *r = Box::new(100);

    let mut rng = rand::rng();
    let n = rng.random_range(0..10);

    if n > 5 {
        println!("{:?}", b);
    }
    else {
        *r = Box::new(200);
        println!("{:?}", r);
    }
}