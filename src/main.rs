#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {

    // statements: declarations

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

    let mut guest_allowed = false;
    let mut confirmed_visitor = Visitor::new("", "");

    let visitors_list = [
        Visitor::new("steve", "Hello Steve!"),
        Visitor::new("bert", "Hello Bert you maniac!"),
        Visitor::new("riz", "Hello Riz, long time no see!"),
    ];

    // expressions: behavior

    println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

    let mut guest = String::new();
    stdin().read_line(&mut guest).expect("Failed to read line:");
    guest = guest.trim().to_lowercase();

    for visitor in &visitors_list {
        if visitor.name == guest {
            guest_allowed = true;
            confirmed_visitor = Visitor::new(&visitor.name, &visitor.greeting);
        }
    }

    if guest_allowed {
        confirmed_visitor.greet();
    } else {
        println!("Sorry, {guest}, you are not on the list.");
    }
}
