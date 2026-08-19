// vector: Vector = A growable array that can hold multiple values of the SAME type.
// Think of it like a shopping list — you can add, remove, and access items, and it automatically grows as you add more!

fn main(){
let mut vec: Vec<i64> = vec![1,2,3,4,5,6];
 vec.len();
 vec[0];
 vec.push(6);
 vec.remove(0);
 println!("{:?}", vec);
}