// Enu = Short for Enumeration = A way to define a type by listing all possible values it can be.
// like a dropdowm nenu with fixed options.

#[derive (Debug)]
enum Myenum{
    A,
    B(i32),
    C{x:i32, y:i32}
}

fn main(){
    let a: Myenum = Myenum:: A;
    let b: Myenum = Myenum:: B(5);
    let c: Myenum = Myenum :: C{x: 10 , y:20};
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}" ,c);



if let Myenum ::B(value)= b {
    println!("{}", value);
}
if let Myenum ::C{x,y}=c{
    println!("{} {}" , x,y)
}
}
