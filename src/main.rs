#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

    let mut visitor_name = String::new();
    stdin()
        .read_line(&mut visitor_name)
        .expect("Failed to read line:");
        visitor_name = visitor_name.trim().to_string();
   println!("Hello, {visitor_name}. Welcome.");
}
