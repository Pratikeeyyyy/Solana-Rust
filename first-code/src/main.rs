

// 🦀 RUST IS STRICTLY/STATICALLY TYPED

//variables:
fn main(){
    //unsigned integer
    // u8, u16 , u 32 ,u64 etc
    let unsigned: u16= 10;

    //signed integer
    // i8, i16, i32, i64
    let signed: i8 = -12;

    //float is used for decimal and it accepts only decimal that must have .0
    let float : f32 =1.0;

    println!("unsign: {} sign:{} float {} ", unsigned, signed , float);
    

    //char can only be

    // Strings (use double quotes "")
    let name: &str = "Pratik";      // &str = string slice
    let car: String = String::from("Pagani");  // String = owned string
    let emoji  :char ='🚀';

    println!("Letter: {}, car: {} , emoji{}", name, car , emoji);
    
// For boooleans
    let is_true: bool =true;
    println!(":isTrue :{}", is_true);
}

// ie simple terms
// let x: u32 = 10;        // u32 = unsigned 32-bit integer
// let y: i32 = -5;        // i32 = signed 32-bit integer
// let z: char = 'A';      // char = single character
// let name: &str = "Rust"; // &str = string slice
// ❌ Wrong (Type mismatch)
// rust
// let x: u32 = -5;        // ERROR: u32 can't be negative
// let y: char = "A";      // ERROR: char expects '' not ""
// let z: String = "Rust"; // ERROR: &str != String
