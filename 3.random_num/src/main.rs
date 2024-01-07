use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // rand::thread_rng is the function used to generate random number
    // .gen_range specifies the range for the numbers

    println!("The secret number is: {secret_number}");

    // looping to take inputs
    loop {
        // taking input
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // converting the input to number. The parse converts string to any desired type

        // using expect for error handling
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // using match for error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // comparing the input with the secret number
        // Ordering is a library which has 3 enums to compare numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),

            // breaks the loop
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
