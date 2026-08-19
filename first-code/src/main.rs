
// Result 

// use std:: collections::HashMap;
#[derive(Debug)]  
enum MyError{
    Error1
}
// err, an enum that contains an error code
// ok (value) , A weapper that contains a value

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn main(){
    let divide = divide(4,2);
    // let res= divide2.expect ("We crashed");
    match divide{
        Ok(v)=> println!("{}", v),  // ← FIXED: "ok" → "Ok" (capital O)
        Err(v)=> println!("{:?}" ,v)  // ← Now works because of #[derive(Debug)]
    }
    // if divide.is_ok(){
//     println!("{}", divide.unwrap());
// }
// println!("{}", divide.unwrap());
// println!("{}", divide.unwrap_or(100));
// println!("{}", res)

}