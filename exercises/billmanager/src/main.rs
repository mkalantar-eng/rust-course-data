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
pub mod menu;

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    map: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.map.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.map.values().collect()
    }

    fn remove(&mut self, name: &str) -> Option<Bill> {
        self.map.remove(&name.to_string())
    }

    fn update(&mut self, name: &str, amount: f64) {
        match self.map.get_mut(name) {
            Some(bill) => {
                bill.name = name.to_string();
            }
            None => println!("Bill not found")
        }
    }
}

fn get_input() -> Option<String> {
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Please enter your data again");
    }
    let input = buf.trim().to_string();
    if input == "" {
        return None;
    }
    Some(input)
}
fn get_bill_amount() -> Option<f64> {
    println!("Amount");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if input == "" {
            return None;
        }

        let amount: Result<f64, _> = input.parse();
        match amount {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }
    fn show() {
        println!();
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bills");
        println!("4. Update bill");
        println!();
        println!("Enter selection: ");
    }
}
fn main() {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let input = get_input().expect("No data entered");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => return,
        }
    }
}
