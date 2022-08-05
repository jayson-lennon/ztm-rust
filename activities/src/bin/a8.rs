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
    Cola,
    Sprite,
    Rum
}

struct Drink {
    flavor: Flavor,
    ml: f64
}

fn display_drink(drink: Drink) {
     match drink.flavor {
        Flavor::Cola => println!("{:?}", "cola"),
        Flavor::Sprite => println!("{:?}", "sprite"),
        Flavor::Rum => println!("{:?}", "rum"),
    }

    println!("{:?}", drink.ml)
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Cola,
        ml: 33.3
    };

    display_drink(drink)
}
