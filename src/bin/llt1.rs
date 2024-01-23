// Just a saved-off exercise demonstrating linked lists to a friend
use std::collections::LinkedList;

fn print_list(pert_data: &LinkedList<String>) {
    println!("{pert_data:#?}");
}

fn main() {
    for n in 0..5 {
        println!("{} - Hello, World!", n);
    }

    let mut list = LinkedList::from(["1".to_string(), "2".to_string(), "3".to_string()]);
    print_list(&list);

    list.push_front("butts".to_string());
    print_list(&list);

    // run tests
    nothin();
}

// #[test]
fn nothin() {
    println!("The test is running!");
    assert!(true);

    // function that takes a linked list as an argument and prints it in order
}
