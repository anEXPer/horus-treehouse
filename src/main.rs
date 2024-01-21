#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::single_match_else)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    AcceptProbationary,
    AcceptWithNote { note: String },
    Refuse,
    RefuseWithNote { note: String },
}

fn main() {
    // statements: declarations

    #[derive(Debug)]
    struct Visitor {
        name: String,
        age: i8,
        action: VisitorAction,
    }

    impl Visitor {
        fn new(name: &str, age: i8, action: VisitorAction) -> Self {
            Self {
                name: name.to_lowercase(),
                age,
                action,
            }
        }
        fn greet(&self) {
            match &self.action {
                VisitorAction::AcceptProbationary => {
                    println!("Probationary access approved!");
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
            self.check_age();
        }
        fn check_age(&self) {
            if self.age < 21 {
                println!("Do not order alcohol, as you are underage.");
            }
        }
    }

    // statements: variables

    let mut visitors_list = vec![
        Visitor::new(
            "steve",
            40,
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Steve!"),
            },
        ),
        Visitor::new(
            "bert",
            14,
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Bert you maniac!"),
            },
        ),
        Visitor::new(
            "riz",
            40,
            VisitorAction::AcceptWithNote {
                note: String::from("Hello Riz, long time no see!"),
            },
        ),
        Visitor::new("pat", 90, VisitorAction::Refuse),
        Visitor::new(
            "liz",
            40,
            VisitorAction::AcceptWithNote {
                note: String::from("Long time no see, Liz!"),
            },
        ),
        Visitor::new(
            "patty-g",
            30,
            VisitorAction::RefuseWithNote {
                note: String::from("I'm not impressed, Pat."),
            },
        ),
    ];

    // expressions: behavior

    loop {
        println!("This is an automated treehouse. Govern Yourself Accordingly. IDENTIFY YOURSELF.");

        let mut raw_name_input = String::new();
        stdin()
            .read_line(&mut raw_name_input)
            .expect("Failed to read line:");
        let name_input = raw_name_input.trim().to_lowercase();

        let confirmed_visitor = visitors_list
            .iter()
            .find(|visitor| visitor.name == name_input);
        match confirmed_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if name_input.is_empty() {
                    println!("Input empty - exiting.");
                    break;
                }
                println!(
                    "Welcome, {name_input}! As it is your first time, you will be added to the probationary list.",
                );
                println!("What is your age, as an integer please?",);
                let mut raw_age_input = String::new();
                stdin()
                    .read_line(&mut raw_age_input)
                    .expect("Failed to read line:");
                let age_input: i8 = raw_age_input.trim().parse().unwrap();

                let new_visitor =
                    Visitor::new(&name_input, age_input, VisitorAction::AcceptProbationary);

                println!("Thank you, {name_input}! Please enjoy your visit.",);
                new_visitor.check_age();
                visitors_list.push(new_visitor);
            }
        }
        println!();
    }
    println!("Final visitor list:");
    println!("{visitors_list:#?}");
}
