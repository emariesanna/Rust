fn main() {
    let r:&i32;
    {
        let mut v = vec![1, 2, 3];
        r = & v[1];
        v.push(4);
        println!("{:?}", r);
        println!("{:?}", v);
    }
}