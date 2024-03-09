fn main() {
    println!("Hello, world!");
    first();
    second();
    third();
}
fn first(){
    let mut x =0;
    loop{
        x+=1;
        println!("x = {}",x);
        if x ==5{
            println!("go the value breaking the loop!");
            break;
        }
    }
}
fn second(){
    let mut number=0;
    while number <11{
        println!("{}",number);
        number+=1;
    }
}
fn third(){
    let n=101;
    let  _x=0;
    for x in 1..n{
        println!("{}",x);
    }
}