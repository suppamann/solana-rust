# cargo run --bin day9

# ownership
let a = 1213;
let b = a; 
print(a) --> ERR:  a is cleared and b is the owner.

# Borrow : video 1 --> 2:50:00 rewatch
passing by reference
you are allowed only a single mutable reference at a time.


# passing values

fn main(){
    let a =9;
    let b =9;
    add(a,b) --> passed by value, actually creates a new copy. this is in case of datatypes that goes in the stack, string would behave different.
    print (a) --> if it was string this would've thrown ERR as ownership changed to add(), no error if passed by ref add(&a,&b)
}

fn add(a,b){
    print(a+b)
}
// you can do .clone() when passing the value, which creates a deep copy, but its not the original design.
