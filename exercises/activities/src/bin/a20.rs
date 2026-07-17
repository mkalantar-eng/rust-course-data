// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn print(&self) {
        match self {
            PowerState::Off => println!("Turning computer off"),
            PowerState::Sleep => println!("Go to sleep"),
            PowerState::Reboot => println!("Reboot the computer..."),
            PowerState::Shutdown => println!("Shutting-down the computer..."),
            PowerState::Hibernate => println!("Go to hibernate"),
        }
    }
    fn from_str(text: String) -> Option<Self> {
        match text.to_lowercase().as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

fn main() {
    println!("Enter computer power state:");
    match get_input() {
        Ok(input) => match PowerState::from_str(input) {
            Some(state) => state.print(),
            None => println!("Unknown command!"),
        },
        Err(msg) => println!("Error: {}", msg),
    }
}
