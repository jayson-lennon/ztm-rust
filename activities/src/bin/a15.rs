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

enum Tickets {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let vec: Vec<Tickets> = vec![
        Tickets::Backstage(String::from("Jan"), 32.9),
        Tickets::Vip(String::from("Adam"), 44.3),
        Tickets::Standard(13.3),
    ];

    for ticket in vec {
        match ticket {
            Tickets::Backstage(name, price) => {
                println!("Backstage ticket for {name} costs {price}")
            }
            Tickets::Standard(price) => println!("Standard ticket costs {price}"),
            Tickets::Vip(name, price) => println!("VIP ticket for {name} costs {price}"),
        }
    }
}
