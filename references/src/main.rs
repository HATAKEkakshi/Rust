fn main() {
    println!("Reference practice!");
    let mut a = 10;
    {
        let z=&mut a;
        *z+=1;
        println!("the value for z is -{}",z);
    }
}
