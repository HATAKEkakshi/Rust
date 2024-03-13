enum Food{
    Oil,
    Ingredients,
    Recipes,
    Taste
}
fn foodfunction(eat:Food){
match eat{
    Food::Oil =>{
        println!("The oil should be healthy oil");
    }
    Food::Ingredients =>{
        println!("The ingredients should be fresh");
    }
    Food::Recipes =>{
        println!("Its can be changed as per user requiremnts");
    }
    Food::Taste =>{
        println!("The taste should be delightful");
    }
}
}
fn main() {
    println!("Advance enumetation!");
    foodfunction(Food::Taste);
    foodfunction(Food::Ingredients);
    foodfunction(Food::Oil);
    foodfunction(Food::Recipes);
}
