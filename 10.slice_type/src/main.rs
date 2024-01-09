// fn first_word(s: &String) -> usize {

//     // converts string to bytes array
//     let bytes = s.as_bytes();


//     // bytes.iter() iterates over the array
//     // enumerate wraps the result of iter and returns each element as part of a tuple instead
//     // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. 
//     // (i,&item ) is the tuple returned from enumerate

//     for (i, &item) in bytes.iter().enumerate() {
//         // b' ' , b is for byte literal
//         if item == b' ' {
//             // if we found a space we will return its index otherwise the length
//             return i;
//         }
//     }
//     s.len()
// }

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{word}");
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}


// let s = String::from("hello");

// let slice = &s[0..2];
// let slice = &s[..2];
// By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:

// let s = String::from("hello");

// let len = s.len();

// let slice = &s[3..len];
// let slice = &s[3..];
// You can also drop both values to take a slice of the entire string. So these are equal:

// let s = String::from("hello");

// let len = s.len();

// let slice = &s[0..len];
// let slice = &s[..];



fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
