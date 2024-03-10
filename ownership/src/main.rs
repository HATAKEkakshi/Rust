fn main() {
    println!("Hello, world!");
    ownership();
}
fn ownership(){
    let a =String::from ("hemant kumar");
    let b=a.clone();
    println!("{}-----{}",a,b);
}