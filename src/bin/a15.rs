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

// * Use an enum for the tickets with data associated with each variant
enum Tickets {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}
fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Tickets::Backstage(50.0, "John".to_owned()),
        Tickets::Standard(15.0),
        Tickets::Vip(30.0, "Mike".to_owned()),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Tickets::Backstage(price, holder) => {
                println!("Backstage Ticket Holder: {:?}, price: {:?}", holder, price)
            }
            Tickets::Standard(price) => println!("Standard ticket price: {:?}", price),
            Tickets::Vip(price, holder) => {
                println!("Vip Ticket Holder: {:?}, price: {:?}", holder, price)
            }
        }
    }
}
