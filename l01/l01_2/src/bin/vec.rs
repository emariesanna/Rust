fn main() {
 let mut v1 =vec![1, 2, 3, 4, 5];

 let mut v2 = vec![0;8];

 v1[0] = 100;
 v2[1] = 5;

 println!("v1.size: {:?} v1.capacity: {:?}", v1.len(), v1.capacity());
 println!("v2.size: {:?} v2.capacity: {:?}", v2.len(), v2.capacity());

 println!("{:?} \n{:?}", v1, v2);
}