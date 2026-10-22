// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    pomarancza,
    cytryna,
    jablko,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    let flavor_name: &str = match drink.flavor {
        Flavor::cytryna => "cyntryna",
        Flavor::pomarancza => "pomarancza",
        Flavor::jablko => "jablko",
    };
    println!("Drink flavor {} and ounces {}", flavor_name, drink.ounces);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::cytryna,
        ounces: 34.3,
    };
    print_drink(drink);
}
