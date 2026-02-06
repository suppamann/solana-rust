use std::collections::HashMap;
fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // scores.get("Blue") returns an Option (it might be null!)
    println!("{:?}", scores.get("Blue")); // {:?} --> debug formatter
}