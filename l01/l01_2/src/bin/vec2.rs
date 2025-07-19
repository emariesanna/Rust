fn main() {
 let mut v1 =vec![1, 2, 3];
 println!("v1.size: {:?} v1.capacity: {:?}", v1.len(), v1.capacity());

 v1.push(4);
 println!("v1.size: {:?} v1.capacity: {:?}", v1.len(), v1.capacity());
 v1.push(5);
 v1.push(6);
 v1.push(7);
 println!("v1.size: {:?} v1.capacity: {:?}", v1.len(), v1.capacity());

 while v1.len() > 0 {
  v1.pop();
 }
 println!("v1.size: {:?} v1.capacity: {:?}", v1.len(), v1.capacity());
}