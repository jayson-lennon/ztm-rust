#![allow(unused_variables)]

fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);

    let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");

    let state = favorites.2;
    let place = favorites.5;
}
