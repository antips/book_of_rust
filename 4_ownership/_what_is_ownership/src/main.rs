fn main() {
    immutable_vs_mutable_string();
    ownership_shift();
    ownership_and_functions();
}

fn immutable_vs_mutable_string() {
    let _s = "hello"; // Immutable string
    let mut s = String::from("hello"); // Mutable string
    s.push_str(", world!"); // push_str() adds a litteral string

    println!("{}", s); // Cela va afficher `hello, world!`
}

fn ownership_shift() {
    let x = 5;
    let _y = x; // Value of x is COPIED in _y, because it is a simple and primitive type.

    let s1 = String::from("hello");
    let _s2 = s1; // The string is NOT copied, because the data is in the HEAP.
                  // What is copied in s2 is the pointer address, the capacity
                  // and the size of the actual string.

    // println!("{}, world!", s1);  // Does NOT work, because s1's value has been
    //BORROWED by _s2

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
