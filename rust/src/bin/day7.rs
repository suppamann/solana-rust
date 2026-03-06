fn main(){
    let mut s = String::from("Hello Achu");
    update_word(s);
    s.push_str("Allo"); // err mutable reference moved.
}

fn update_word(a:String){
    let mut s =a;
    s.push_str(" My Forever Love");
    print!("{}",s);
    // return a;
}