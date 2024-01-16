#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    // statements: declarations

    #[derive(Debug)]
    struct Visitor {
        name: String,
        greeting: String,
    }

    impl Visitor {
        fn new(name: &str, greeting: &str) -> Self {
            Self {
                name: name.to_lowercase(),
                greeting: greeting.to_string(),
            }
        }
        fn greet(&self) {
            println!("{}", self.greeting);
        }
    }

    // statements: variables

    let visitors_list = vec![
        Visitor::new("steve", "Hello Steve!"),
        Visitor::new("bert", "Hello Bert you maniac!"),
        Visitor::new("riz", "Hello Riz, long time no see!"),
    ];

    // expressions: behavior

    loop {
        println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

        let mut guest = String::new();
        stdin().read_line(&mut guest).expect("Failed to read line:");
        guest = guest.trim().to_lowercase();

        let confirmed_visitor = visitors_list.iter().find(|visitor| visitor.name == guest);
        match confirmed_visitor {
            Some(visitor) => visitor.greet(),
            None => println!("Sorry, {guest}, you are not on the list."),
        }
        break;
    }
}
