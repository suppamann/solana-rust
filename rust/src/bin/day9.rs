//implement struct

struct User {
    name: String,
    age: u8,
}

impl User {
    fn returner(&self) -> u8{
        self.age
        //when there is no semi colon, its equivalent to return self.age
    }
}
fn main(){
    let user = User{
        name:String::from("Hello Achu"),
        age:27,
    };

    print!("The age of {} is: {}",user.name, user.returner());
}