fn main() {
  let mut i = 32;
  
  let r = &mut i;
  println!("{}", i);// Problema!
  
  *r = *r+1;    
  println!("{}", *r);
}
