fn main(){
    heap_fn();
}

fn heap_fn(){
    let s1 = String::from("Hello Achu");
    let s2 = String::from("🌏");
    let combined = format!("{} {}", s1,s2 );
    print! ("Combined string {}", combined);
}