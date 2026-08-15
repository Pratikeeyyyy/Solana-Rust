// function and its types
 // public function (pub  is used to make a function public) and default function is itself automatic private function.

 fn main(){
    println!("{}", is_even(2));
 }

 pub fn is_even(num: u8)-> bool{
    let digit: u8 = num % 2;
    digit ==0
 }

 // Public function
pub fn public_fn() {}

// Public struct with private fields
pub struct User {
    pub name: String,    // Public field
    age: i32,            // Private field
}

// Public enum (all variants are automatically public)
pub enum Color {
    Red,
    Blue,
}

// Public trait
pub trait Animal {
    fn make_sound(&self);
}


//  mutability:means can something change? In Rust, everything is immutable (can't change) by default. You have to explicitly say you want something to be mutable.

 fn main(){
    let  mut number = 5;
    let number = 1;
    println!("Number is :{}",number)
 }

fn main() {
    // IMMUTABLE array can't change
    let numbers = [1, 2, 3];
    numbers[0] = 10;  // ❌ ERROR! Can't change
    
    // MUTABLE array  can change
   
    let mut numbers = [1, 2, 3];
    numbers[0] = 10;  // ✅ Allowed!
    println!("{:?}", numbers);  // → [10, 2, 3]
}

fn main() {
    // IMMUTABLE tuple  can't change
    let person = ("Alice", 25);
    person.1 = 30;  // ❌ ERROR!
    
    // MUTABLE tuple  can change
    let mut person = ("Alice", 25);
    person.1 = 30;  // ✅ Allowed!
    println!("{:?}", person);  // → ("Alice", 30)
}


// Variable (can be mutable or immutable)
let x = 5;           // Immutable
let mut y = 10;      // Mutable

fn main() {
    // This is SHADOWING (creating new variable)
    const x = 5;
    const x = x + 1;    // New variable, old one is gone
    const x = x * 2;
    println!("{}", x);  // → 12
    
    // This iss MUTABILITY (changing existing variable)
    const mut y = 5;
    y = y + 1;        // Changing the same variable
    y = y * 2;
    println!("{}", y);  // → 12
}