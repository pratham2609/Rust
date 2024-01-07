// fn main() {
//     another_function(5);
// }


// have to pass the type of variable in the parameter
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements are instructions that perform some action and do not return a value. ---just like void
// Expressions evaluate to a resultant value. Let’s look at some examples.


// Statement 
// fn main() {
//     let y = 6;
// }


// Expression
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }


// Functions with return values

// We don’t name return values, but we must declare their type after an arrow (->). it is the type of returned value
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1  // the returning value . Don't put semicolon
}