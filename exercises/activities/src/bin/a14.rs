// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: u32,
    name: String,
    fav_color: String,
}

fn print(p: &str) {
    println!("{p}");
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 10,
            name: "ali".to_owned(),
            fav_color: "blue".to_owned(),
        },
        Person {
            age: 12,
            name: "hasan".to_owned(),
            fav_color: "red".to_owned(),
        },
        Person {
            age: 5,
            name: "kamal".to_owned(),
            fav_color: "yellow".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age < 11 {
            // * The name and colors should be printed using a function
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
