use crate::{Bills, Bill, get_input, get_bill_amount};
pub(crate) fn add_bill(bills: &mut Bills) {
    println!("Bill name");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(input) => input,
        None => return,
    };

    let bill = Bill { name, amount };
    bills.add(bill);
}

pub(crate) fn view_bills (bills: &Bills) {
    bills.get_all().iter().for_each(|bill| println!("{:?}", bill))
}

pub(crate) fn remove_bill(bills: &mut Bills) {
    println!("Bill name");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    
    match bills.remove(&name) {
        Some(bill) => println!("Successfully removed {:?}", bill),
        None => println!("No bill with name {name}")
    }
}
pub(crate) fn update_bill(bills: &mut Bills) {
    println!("Bill name");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(input) => input,
        None => return,
    };
    bills.update(&name, amount);
}
