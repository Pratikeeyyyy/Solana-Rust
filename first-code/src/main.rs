fn main(){
let first : &str=" Hello everyone";

let mut second: String= String::from ("How are you");

let slice =&second[..6];// slcie form index starting to 6( excluding 6)
println!("slice: {}", slice);
println!("slice length:{}", slice.len());
second.push('1');
// string.push_first("!");// old
second.insert(0, '!');
println!("After inserting : {}", second);
second= second.replace("How", "Guys");
println!("After inserting : {}", second);

}