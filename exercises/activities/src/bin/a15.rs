// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

struct Ticket {
    event: String,
    price: i32,
    ticket_type: TicketType,
}

fn main() {
    let tickets = vec![
        Ticket {
            event: "ev1".to_owned(),
            price: 1333,
            ticket_type: TicketType::Standard,
        },
        Ticket {
            event: "ev1".to_owned(),
            price: 2333,
            ticket_type: TicketType::Backstage("ali hasani".to_owned()),
        },
        Ticket {
            event: "ev other".to_owned(),
            price: 3333,
            ticket_type: TicketType::Vip("jamal kamali".to_owned()),
        },
    ];

    for ticket in tickets {
        match ticket {
            Ticket {
                event,
                price,
                ticket_type,
            } => {
                if event == "ev1" {
                    println!("{price:?} {ticket_type:?}")
                }
            }
        }
    }
}
