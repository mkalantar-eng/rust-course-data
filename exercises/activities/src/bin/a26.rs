// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

fn main() {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    println!("local time: {}", local.time());

    let custom = local.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("local time with custom format: {}", custom);
}
