// match compares a value against multiple patterns and executes the code for the first matching pattern.



fn main(){
let i =7;
match i {
    0=> println!("0"),
    1|2|3 => println!("1,2"),
    4..=5=> println!("***"),
    _=> println!("default")
}

// eg 2
   let is_raining = true;
    
    match is_raining {
        true => println!("Bring an umbrella! ☂️"),
        false => println!("Enjoy the sunshine! ☀️"),
    }

    // eg3
      let score = 85;
    
    match score {
        0..=59 => println!("Grade: F"),
        60..=69 => println!("Grade: D"),
        70..=79 => println!("Grade: C"),
        80..=89 => println!("Grade: B"),
        90..=100 => println!("Grade: A"),
        _ => println!("Invalid score!"),
    }
    // Output: Grade: B

    // eg4
    let name = "Alice";
    
    match name {
        "Alice" => println!("Hello Alice!"),
        "Bob" => println!("Hi Bob!"),
        _ => println!("Who are you?"),
    }
    // Output: Hello Alice!
}