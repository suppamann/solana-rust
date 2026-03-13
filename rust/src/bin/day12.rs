//converting tuple -> hashmap

use std::collections::HashMap;

fn group_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm: HashMap<String, i32> = HashMap::new();
    for (key,val) in vec{
        hm.insert(key,val);
    }
    hm
}

fn main() {
    let input_vec = vec![(String::from("Achu"),34),(String::from("Abhi"),34)];
    let hm = group_by_keys(input_vec);
    println!("{:?}", hm);
}
