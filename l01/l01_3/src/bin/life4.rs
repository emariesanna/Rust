fn cambia (par: &mut i32, val: i32)
{
    *par = val;
}
fn main() {
    let r:&mut i32;
    {
        let mut v = vec![1, 2, 3];
        r = &mut v[1];
        println!("{:?}", v);
        cambia(r, 100);
    }
}