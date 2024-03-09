fn main() {
    println!("Hello, world!");
    function();
    singleparameter(323);
    secondfunction(42,"hemant");
    ex();
    let xy=returnfunction(32,32);
    println!("the value return is -{}",xy);
}

fn function(){
    println!("This is my example for function");
}
fn singleparameter(x : i32){
    println!("the value for x is - {}",x);
}
fn secondfunction(x : i32 ,y : &str){
    println!("the value for x is -{} , the value for y is - {}",x,y);
}
fn ex(){
    let y = {
        let x=9;
        x+1
    };
    println!("The value for y is - {}",y);
}
fn returnfunction(x:i32,y:i32) -> i32 {
    x+y
}