// struct
fn main(){
let name = String::from("Anaconda");
let creature = animal{name, attribute:69 };
creature.print_name();

   println!("Lays egg: {}", creature.lay_egg());
    println!("No egg: {}", creature.no_egg());
}
struct animal {
    name :String,
    attribute:u64,
}

impl animal{
    fn print_name (&self){
        println!("{}", self.name);
    }
}

// traits
trait Bird {
    fn lay_egg(&self) -> bool;  // No default implementation
    fn no_egg(&self) -> bool;   // No default implementation
}

impl Bird for animal {
    fn lay_egg(&self) -> bool {
        true  // Anaconda lays eggs
    }
    
    fn no_egg(&self) -> bool {
        false  // Anaconda does lay eggs, so no_egg is false
    }
}
