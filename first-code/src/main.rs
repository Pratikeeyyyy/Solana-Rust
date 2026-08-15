fn main (){
   //simple array
    let array :[u8; 3]=[1, 2, 3]; // u8type of  array of 3 item
    let array_2:[u8; 5]=[100; 5]; // u8 type of array where each item has item 100 up to 5th place
    println!("index:{}, length:{}", array[0], array_2.len());

    // printing the structure of array and other objects
    println!("{:?}", array); // pprint whole array
    println!("{}", array[1]);// print specific item of resepctive index
    println!("{}", array[0]);
    println!("{}", array[2]);
println!("{}", array_2[4]);

}