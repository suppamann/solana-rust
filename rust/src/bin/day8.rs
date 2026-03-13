//simple struct
struct User{
    name: String,
    age: u16,
    active: bool
}

fn main(){
    let name =  String::from("Achu");
    let  user = User{
        name,
        active: true,
        age: 34,
    };
    print!("{}",user.name);
}