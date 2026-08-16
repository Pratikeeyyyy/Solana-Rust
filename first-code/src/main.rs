fn main(){

    let array =[0, 1, 2, 3]; // array of 4 items

    // Creating a slice from index 1 to 3 (excludes 3)
    let slice =&array [1..3];// correct range of syntaxx from 1  to 2

    //passing array refrence and sllice 
    borrowing_slice(&array, slice);
}

// First parameter: &[u8] (borrowed slice of the whole array)
// Second parameter: &[u8] (borrowed slice of part of array)
 fn borrowing_slice(array: &[u8], slice: &[u8]){
    println!("array:{:?}", array);
    println!("slice:{:?}", slice);
    println!("lenght of slice: {}", slice.len());
    println!("First two element of slice {},{}", slice[0], slice[1]);


 }