fn main() {
 let mut a = [ 1, 2, 3, 4 ];

 let s2 = &mut a[0..2];

 s2[0] = 10;

 println!("array {:?}", a);
 println!("slice {:?}", s2);
}
