// fn main() {
//     let mut a = String::from("Hello Achu");
//     let b = "string";
//     let c = &mut a;
//     let c = &mut a;

//     something(&mut a);
//     print!("{a} {b}");
// }

// fn something(a: &mut String) {
//     a.push_str("!!!!");
// }
fn main() {
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}

fn dangle() {
    let s = String::from("hello");

    &s //this is wraang
}
