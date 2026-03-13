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


# string storage
let mut achu =  String::from("Hello World");

> here the variable achu, on the stack will not only have the memory location where the first byte (index[0]) is stored on the heap, it will also have the LENGTH and CAPACITY as well.
Heap memory for a string is contigous, so when you have the index[0] addresss and the length, you can retrieve the entire string.

> when the string is updated achu.push_str(string:"!!!!");
if the new content is within the capacity, then only the length needs to be updated.
if it exeeds capacity, then OS is requested to provide a larger block of heap memory, capacity, pointer, length are updated and old memory is deallocated.

# Ownership
let achu = String::from("Hello World");
let mia = achu;
print!("{achu}"); // err as ownership of Hello World has changed to mia.


When you execute let mia = achu;, Rust performs a shallow copy of the 24-byte stack structure (Pointer, Length, Capacity) from achu to mia.

If this were C++, you would now have two variables pointing to the same memory (Aliasing), which leads to a Double Free error when both try to deallocate the memory at the end of the scope.

# Move

fn main(){
    let a = String::from("Hello Achu");
    {
        let a1 = a;
        print!("Inside: {a1}");
    }
    print!("Outside: {a}"); // regardless of the scope the new owner the heap memory is a1 and it has already been deallocated when the scope of a1 ended.
}

let a1 = a; and a fn some_fn(a); works the same way, as ownership has been moved

# Borrow

let achu =String::from("Hello World");
let mia = &achu;

achu is still the owner, but achu and mia both points to the same memory address, borrowing.

need to add mut, if value is to be mutated.