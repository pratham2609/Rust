// fn main() {
//     // Data types which can beb copied

//     // All the integer types, such as u32.
//     // The Boolean type, bool, with values true and false.
//     // All the floating-point types, such as f64.
//     // The character type, char.
//     // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    
    
    
//     // Strings size is not calculated at compile time and memory is allocated at runtime
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
//     let x = 5;
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }


// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.



fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}