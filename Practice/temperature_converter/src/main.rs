use std::io;
fn converter_to_c(x: u32) -> u32 {
    (x - 32) * 5 / 9
}
fn converter_to_f(x: u32) -> u32 {
    (x * 9 / 5) + 32
}

fn main() {
    println!("Temperature converter");
    println!("Type \"quit\" to end the program");

    loop {
        let mut temp = String::new();
        let mut scale = String::new();
        println!("\n Type \"f\" for Farenheit and \"c\" for Celsius:");
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");
        if scale.trim() == "quit" {
            break;
        }
        println!("\nEnter a temperature:");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        if scale.trim() == "quit" {
            break;
        }

        let temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if scale.trim() == "f" {
            println!("{}°C", converter_to_c(temp));
        } else {
            println!("{}°F", converter_to_f(temp));
        }
    }
}
