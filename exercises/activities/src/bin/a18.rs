// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: u32,
}

fn can_make_restricted_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("Restricted purchases require that the age of the customer is at least 21".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let old = Customer {
        name: "Nadya".to_owned(),
        age: 67,
    };

    let teen = Customer {
        name: "Noha".to_owned(),
        age: 17,
    };

    match can_make_restricted_purchase(&old) {
        Ok(_) => println!("Ok to purchase {}", old.name),
        Err(msg) => println!("{msg} {}", old.name),
    };

    match can_make_restricted_purchase(&teen) {
        Ok(_) => println!("Ok to purchase {}", teen.name),
        Err(msg) => println!("{msg} {}", teen.name),
    };
}
