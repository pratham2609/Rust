// fn main() {
//     let s1 = String::from("hello");
//     // if we dont want to transfer the ownership
//     // we can borrow it or have its reference to use it
//     let len = calculate_length(&s1);

//     // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     // Cannot modify something we are borrowing
//     s.len()
// }



// Modifications using mutable reference
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// The restriction preventing multiple mutable references to the same data at the same time allows for mutation 
// but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages 
// let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. 
// A data race is similar to a race condition and happens when these three behaviors occur:

// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.

// to prevent it do 
// let mut s = String::from("hello");

// {
//     let r1 = &mut s;
// } // r1 goes out of scope here, so we can make a new reference with no problems.

// let r2 = &mut s;


// We also cannot have a mutable reference while we have an immutable one to the same value.