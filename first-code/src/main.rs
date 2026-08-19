// HashMap = A key-value store where you can look up values by their keys. Like a dictionary!

// Think of it like a real dictionary — you look up a word (key) to find its definition (value).

//  importin the hasmap form lbrary
use std::collections::HashMap;

fn main(){
    let mut egmap = HashMap::new();

    egmap.insert(0,"Hello");
    egmap.insert(1,"Everyone");
    println!("{:?}", egmap);

    match egmap.get(&0){
        Some(str) => println!("{}", str),
        None=>println!("Doesnt exist in map "),}

 match egmap.get(&2){
        Some(str) => println!("{}", str),
        None=>println!("Doesnt exist in map "),}

        egmap.remove(&0);
        println!("{:?}",egmap);
    }
