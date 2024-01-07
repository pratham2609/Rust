// Variables Constants and Mutability

// if we don't want to use a variable start its name with _

// fn main() {
//     // mutable variables can be changed
//     // immutable cannot be changed

//     // By default the variable is immutable
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {x}");

//     // declaring constants

//     // constants cannot be mutable
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!(
//         "The value of THREE_HOURS_IN_SECONDS is: {}",
//         THREE_HOURS_IN_SECONDS
//     );
// }

// Shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
