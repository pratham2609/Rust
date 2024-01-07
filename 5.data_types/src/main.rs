// Floating numbers

// fn main() {
//     //default value of floating point is f64
//     let x = 2.0; // f64

//     let _y: f32 = 3.0; // f32

//     println!("The value of x is: {x}");
//     println!("The value of y is: {_y}");
// }

// Boolean Type
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
//     println!("The value of t is: {}", t);
//     println!("The value of f is: {}", f);
// }

// Character Type
// Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
// Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust
// fn main() {
//     let _c = 'z';
//     let _z: char = 'ℤ'; // with explicit type annotation
//     let _heart_eyed_cat = '😻';
// }

// Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

// Tuples
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.

// fn main() {
//     // // a tuple with 3 different types
//     // let tup: (i32, f64, u8) = (500, 6.4, 1);

//     // // destructuring the tuple
//     // let (x, y, z) = tup;

//     // println!("The value of y is: {y}");

//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     // getting the values without destructuring
//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// Arrays
// fn main() {
//     let a1 = [1, 2, 3, 4, 5]; // undeclared type
//     let a2: [i32; 5] = [1, 2, 3, 4, 5]; // declared type and size

//     // You can also initialize an array to contain the same value for each element by specifying the initial value,
//     // followed by a semicolon, and then the length of the array in square brackets,
//     let a3 = [3; 5]; // makes an array of [3, 3, 3, 3, 3]

// }


// Accessing array elements
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
