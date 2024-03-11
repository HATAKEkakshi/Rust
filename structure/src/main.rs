struct User{
    name:String,
    company:String,
    age:u32
}
struct Paswword{
    filename:String,
    username:String,
}
fn main() {
    println!("structure learning!");
    let mut un1=User{
        name: String::from("Hemant kumar"),
        company:String::from("LearnExpo"),
        age:21,
    };
    un1.age=19;
    println!("the name of the person is -{},the age is -{},the comapy name is-{}",un1.name,un1.age,un1.company);
    userpass();
}
fn userpass(){
    let ut1=Paswword{
        filename:String::from("hemant kumar"),
        username:String::from("hatake kakashi")
    };
    println!("the filename is -{:?},the usesrname is -{:?}",ut1.filename,ut1.username);
}