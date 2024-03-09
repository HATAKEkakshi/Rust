use std::io;
fn main() {
    println!("Data types in rust!");
    test(356);
    let mut tup: (i32,u8,f64)=(32,64,45.9);
    println!("{:?}",tup);
    println!("{}",tup.2);
    tup.0 = 23;
    println!("{:?}",tup);
    let mut input =String::new();
    println!("yo say something");


}
fn test(x:i128){
    println!("{}",x);
}

