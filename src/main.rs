use std::io::stdin;

fn main() {
    println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

    let mut visitor_name = String::new();
    stdin()
        .read_line(&mut visitor_name)
        .expect("Failed to read line:");
}
