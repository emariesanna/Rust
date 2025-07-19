fn main() {
 let mut v =Vec::<i32>::new();
 println!("ptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());

 v.push(1);

 println!("ptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());

 v.push(2);
 v.push(3);
 v.push(4);
 println!("ptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());

 v.push(5);

 println!("ptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());

 v.shrink_to_fit();
 println!("Shrinking\nptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());


 let a = v.remove(1);
 println!("Removing\nptr: {:p} C {} S {}", v.as_ptr(), v.capacity(), v.len());
 println!("Removed {:?} ", a);
}
