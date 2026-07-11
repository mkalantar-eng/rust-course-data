// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:

// * Use an enum to create different flavors of drinks
enum DrinkFlavor {
    Coffee,
    Tea,
    Milk,
    Juice,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: DrinkFlavor,
    ounce_info: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print(drink: Drink) {
    match drink.flavor {
        DrinkFlavor::Coffee => println!("Drink flavor: Coffee"),
        DrinkFlavor::Tea => println!("Drink flavor: Tea"),
        DrinkFlavor::Milk => println!("Drink flavor: Milk"),
        DrinkFlavor::Juice => println!("Drink flavor: Juice"),
    }

    println!("fluid ounce information: {:?}", drink.ounce_info);
}
// * Use a match expression to print the drink flavor

fn main() {
    let drink = Drink {
        flavor: DrinkFlavor::Coffee,
        ounce_info: 3.14,
    };

    print(drink);
}
