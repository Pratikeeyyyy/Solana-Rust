// tuples : A is like a mixed box that can hold different types of things together! like char num boolean etc.

 fn main(){

    let person=("ram", 1, true);

        // Access items using .0, .1, .2 (starts at 0)
        println!("Name  :{}" , person.0);
        println!("Rollno:{}" , person.1);
        println!("student:{}" , person.2);
 }
 
      // Empty tuple (called "unit")
    let empty = ();

    // With 2 items
    let pair = ("hello", 42);

    // With 3 items  
    let triple = (10, 20.5, "world");

    // Destructuring (unpacking)
    let (x, y, z) = triple;
    println!("{} {} {}", x, y, z);  // → 10 20.5 world
