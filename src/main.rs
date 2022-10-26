use std::io;
mod temperature_converter;
mod variables;
pub use crate::temperature_converter::temperature_converter::*;
pub use crate::variables::variables::*;
fn main() {
    loop {
        println!("Welcome to Moikapy's Rust Playground!");
        println!("Please select a section to view: hh for help");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        // Variables and Mutability

        match cmd.trim() {
            "v" => {
                variables_section();
                break;
            }
            "t" => {
                println!("Please Enter Degree to Convert");
                let mut degree = String::new();
                io::stdin()
                    .read_line(&mut degree)
                    .expect("Failed to read line");
                let degree: i32 = match degree.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                println!("Is this degree in Fahrenheit or Celsius? (F/C)");
                let mut conversion = String::new();
                io::stdin()
                    .read_line(&mut conversion)
                    .expect("Failed to read line");
                let conversion: char = match conversion.to_lowercase().to_string().trim().parse() {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                converter_manager(degree, conversion);
                break;
            }
            "hh" => {
                println!("v - Variables Section");
                println!("hh - Help");
                println!("q - Quit");
            }
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}
