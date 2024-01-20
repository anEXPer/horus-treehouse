#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::single_match_else)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    RefuseWithNote { note: String },
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
                action,
            }
        }
        fn greet(&self) {
            match &self.action {
                VisitorAction::Accept => {
                    println!("Access approved!");
                    println!("Welcome back, new friend!");
                }
                VisitorAction::Refuse => {
                    println!("You are on the deny list.");
                    println!("Please leave immediately.");
                }
                VisitorAction::AcceptWithNote { note } => {
                    println!("Access approved!");
                    println!("{note}",);
                }
                VisitorAction::RefuseWithNote { note } => {
                    println!("You are on the deny list.");
                    println!("{note}",);
                }
            }
        }
    }

    // statements: variables

    let mut visitors_list = vec![
        Visitor::new(
            "steve",
            "Hello Steve!",
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Steve!"),
            },
        ),
        Visitor::new(
            "bert",
            "Hello Bert you maniac!",
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Bert you maniac!"),
            },
        ),
        Visitor::new(
            "riz",
            "Hello Riz, long time no see!",
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Riz, long time no see!"),
            },
        ),
        Visitor::new("pat", "Get out of here Pat.", VisitorAction::Refuse),
        Visitor::new(
            "liz",
            "Long time no see, Liz!",
            VisitorAction::AcceptWithNote {
                note: String::from("Long time no see, Liz!"),
            },
        ),
        Visitor::new(
            "patty-g",
            "I'm not impressed, Pat.",
            VisitorAction::RefuseWithNote {
                note: String::from("I'm not impressed, Pat."),
            },
        ),
    ];

    // expressions: behavior

    loop {
        println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

        let mut rawinput = String::new();
        stdin()
            .read_line(&mut rawinput)
            .expect("Failed to read line:");
        let input = rawinput.trim().to_lowercase();

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
