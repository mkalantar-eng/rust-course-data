// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:

// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(gi: &GroceryItem) {
    println!("{}", gi.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(gi: &GroceryItem) {
    println!("{}", gi.id);
}

fn main() {
    let gi = GroceryItem {
        id: 1,
        quantity: 5
    };

    display_quantity(&gi);
    display_id(&gi);

}

