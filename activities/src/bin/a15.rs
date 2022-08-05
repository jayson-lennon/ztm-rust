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
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(5.0, "r1oga".to_owned()),
        Ticket::Vip(10.0, "kristin".to_owned()),
        Ticket::Standard(3.2),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Vip(price, holder) => {
                println!("Vip ticket holder {:?}, price {:?}", holder, price)
            }
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket holder {:?}, price {:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket price {:?}", price),
        }
    }
}
