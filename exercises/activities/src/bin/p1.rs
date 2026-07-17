// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be its own function, so you can work on
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Self { name, amount }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

fn display_menu() -> io::Result<String> {
    println!();
    println!("== Manage Bills ==");
    println!("1. Add bill");
    println!("2. Search bills");
    println!("3. View all bills");
    println!("4. Remove bill");
    println!("5. Update bill");
    println!("6. Bill total");
    println!("7. Quit");
    println!();
    println!("Enter selection: ");

    get_input()
}
fn main() {
    let mut bills: Vec<Bill> = vec![];
    loop {
        match display_menu() {
            Ok(selection) => match selection.as_str() {
                "1" => {
                    println!("Enter name: ");
                    if let Ok(name) = get_input() {
                        println!("Enter amount owed: ");
                        if let Ok(amount) = get_input() {
                            bills.push(Bill::new(name, amount.parse().unwrap()));
                        }
                    }
                }
                "2" => {
                    println!("Enter name to search: ");
                    if let Ok(name) = get_input() {
                        bills
                            .iter()
                            .filter(|bill| bill.name == name)
                            .for_each(|bill| println!("{:?}", bill));
                    }
                }
                "3" => {
                    if bills.is_empty() {
                        println!("There are no available bills");
                        continue;
                    }
                    bills.iter().for_each(|bill| println!("{:?}", bill));
                }
                "7" => return,
                _ => println!("Enter number between 1 and 6"),
            },
            Err(msg) => println!("Error: {}", msg),
        }
    }
}
