fn main() {
    println!("Hello, world!");
    function();
    singleparameter(323);
    secondfunction(42,"hemant");
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