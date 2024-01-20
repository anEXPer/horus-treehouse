#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    Refuse,
}

fn main() {
    // statements: declarations

    #[derive(Debug)]
    struct Visitor {
        name: String,
        greeting: String,
        action: VisitorAction,
    }

    impl Visitor {
        fn new(name: &str, greeting: &str, action: VisitorAction) -> Self {
            Self {
                name: name.to_lowercase(),
                greeting: greeting.to_string(),
                action: action,
            }
        }
        fn greet(&self) {
            match &self.action {
                VisitorAction::Accept => println!("{}", self.greeting),
                VisitorAction::Refuse => {
                    println!("You are on the deny list. Please leave immediately.");
                }
            }
        }
    }

    // statements: variables

    let mut visitors_list = vec![
        Visitor::new("steve", "Hello Steve!", VisitorAction::Accept),
        Visitor::new("bert", "Hello Bert you maniac!", VisitorAction::Accept),
        Visitor::new("riz", "Hello Riz, long time no see!", VisitorAction::Accept),
        Visitor::new("pat", "Get out of here Pat.", VisitorAction::Refuse),
    ];

    // expressions: behavior

    loop {
        println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line:");
        input = input.trim().to_lowercase();

        let confirmed_visitor = visitors_list.iter().find(|visitor| visitor.name == input);
        match confirmed_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if input.is_empty() {
                    println!("Input empty - exiting.");
                    break;
                }
                println!(
                    "Welcome, {input}! As it is your first time, you will be added to the list.",
                );
                visitors_list.push(Visitor::new(
                    &input,
                    "Welcome back new friend!",
                    VisitorAction::Accept,
                ));
            }
        }
    }
    println!("Final visitor list:");
    println!("{visitors_list:#?}");
}
