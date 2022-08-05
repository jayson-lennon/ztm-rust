// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Blue,
    Red,
    Green
}

fn  display(color:Color)  {
    return match color {
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
        Color::Green => println!("green")
    }
}

fn main() {
    display( Color::Green);
    display( Color::Blue);
    display( Color::Red);
}
