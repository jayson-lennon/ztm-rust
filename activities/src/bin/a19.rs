// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let store = HashMap::from([("Chair", 5), ("Bed", 3), ("Table", 2), ("Couch", 0)]);

    let mut total = 0;

    for (item, qty) in store {
        total += qty;
        println!("{:?}: {:?}", item, qty)
    }

    println!("Total items in store {:?}", total)
}
