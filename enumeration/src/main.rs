enum Cartypes{
    Hatchback,
    Sedan,
    Suv,
    Muv
}
fn printcar(car:Cartypes){
 match car{
    Cartypes::Hatchback =>{
        println!("Small car in a segment");
    }
    Cartypes::Sedan =>{
        println!("it is family car");
    }
    Cartypes::Suv =>{
        println!("pocket friendly car");
    }
    Cartypes::Muv =>{
        println!("Big brother");
    }
}
}
fn main() {
    println!("Learing enum");
    printcar(Cartypes::Suv);
}
