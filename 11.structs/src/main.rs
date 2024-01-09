// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");      // can change the values only if the struct is mutable

//     let user2 = build_user(String::from("test@gmail.com"), String::from("username"));
//     // Creating Instances from Other Instances with Struct Update Syntax
//     let user3 = User {
//         active: user2.active,
//         username: user2.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user2.sign_in_count,
//     };
//     let user4 = User{
//         email: String::from("hello"),
//         ..user3
//     };
//     println!("{}",user4.username);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }


// Using Tuple Structs Without Named Fields to Create Different Types
// struct Color(i32, i32, i32);  // defining tuple structs
// struct Point(i32, i32, i32);  // defining tuple structs

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// Unit-Like Structs Without Any Fields

// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }



// // Refactoring with tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// can create problem as we know 0th index is height and 1st index is width but can cause problem if we want to do something else like to draw a rectangle




// Refactoring with Structs: Adding More Meaning

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1) // passing refernce with & so that main should have its ownership and can use it further
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// Printing a struct using debug


// Rust does include functionality to print out debugging information,
// but we have to explicitly opt in to make that functionality available for our struct. 
// To do that, we add the outer attribute #[derive(Debug)] just before the struct definition 

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:#?}", rect1);
//     // {:?} provides the debug feature and adding # can make the output more readable
// }



// Adding dbg! macro instead of {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, 
// which prints to the standard output console stream (stdout). 
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, 
    // the width field will get the same value as if we didn’t have the dbg! call there. 
    // We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. 
}