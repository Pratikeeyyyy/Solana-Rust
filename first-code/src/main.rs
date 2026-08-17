fn main(){
   // for loop 
//  let n=10;
//  if n>0{
//     println!("It is greater than 0");
//  }
// else if n<0{
//     println!("It is less than 0");
// }
// else {
//     println!("The number is 0");
// }

// for loop similar to python
// for i in 0..7{
//     println!("{}" , i);
// }


// while loop
let mut  i = 0;
while i<6{
    println!("{}", i);
    i+= 1;
    if i ==3{
        println!("exit");
        break // break or  contine to terminate the execution
    }
}
}