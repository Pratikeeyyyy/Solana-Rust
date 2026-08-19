// option 
// None  is used to indicate failure or lack of values 
// some (value ) is a tuple structure that wraps a value with type T.

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let _divide2: Option<i32> = divide(2, 3);

    // un wrapping a Some varient will extract the value wrapped.
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    // unwrapping a None varient will panic!.
    // println!("{:?} unnwraps to {}" , divide2, divide2.unwrap())
    
    // Call the function here
    Don();
}

fn Don() {
    let score: Option<i32> = Some(85);
    let no_score: Option<i32> = None;
    
    match score {
        Some(value) => println!("Score is: {}", value),
        None => println!("No score available"),
    }
    // Output: Score is: 85
    
    match no_score {
        Some(value) => println!("Score is: {}", value),
        None => println!("No score available"),
    }
    // Output: No score available
}