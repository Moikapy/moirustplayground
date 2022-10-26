use std::io;
mod fibonacci;
mod temperature_converter;
mod variables;
pub use crate::fibonacci::fibonacci::*;
pub use crate::temperature_converter::temperature_converter::*;
pub use crate::variables::variables::*;
fn main() {
    loop {
        println!("Welcome to Moikapy's Rust Playground!");
        println!(
            "I Made This Program To Learn Rust Programming Language And To Share My Code With You!"
        );
        println!("Some Featurs Inspired By The Rust Programming Language Book!");
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
            "fib" => {
                println!("Please Enter Number");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read line");
                let number: i32 = match number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                get_n_fibonacci(number);
                break;
            }
            "12" => {
                for number in 1..13 {
                    match number {
                        1 => {
                            println!(
                                "On the first day of Christmas, my true love sent to me A partridge in a pear tree"
                            )
                        }
                        2 => {
                            println!(
                                "On the second day of Christmas, my true love sent to me Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        3 => {
                            println!(
                                "On the third day of Christmas, my true love sent to me Three french hens Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        4 => {
                            println!(
                                "On the fourth day of Christmas, my true love sent to me Four calling birds Three french hens Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        5 => {
                            println!(
                                "On the fifth day of Christmas, my true love sent to me Five golden rings Four calling birds Three french hens Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        6 => {
                            println!(
                                "On the sixth day of Christmas, my true love sent to me Six geese a-laying Five golden rings Four calling birds Three french hens Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        7 => {
                            println!(
                                "On the seventh day of Christmas, my true love sent to me Seven swans a-swimming Six geese a-laying Five golden rings Four calling birds Three french hens Two turtle doves, and A partridge in a pear tree"
                            )
                        }
                        8 => {
                            println!("August")
                        }
                        9 => {
                            println!("September")
                        }
                        10 => {
                            println!("October")
                        }
                        11 => {
                            println!("November")
                        }
                        12 => {
                            println!("December")
                        }
                        _ => {
                            break;
                        }
                    }
                }
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
