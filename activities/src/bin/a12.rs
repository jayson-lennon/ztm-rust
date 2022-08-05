// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue,
    Orange,
}

struct Box {
    color: Color,
    dimensions: (i32, i32),
    weight: i32,
}

impl Box {
    fn create(color: Color, dimensions: (i32, i32), weight: i32) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn print(&self) {
        println!("{:?} {:?} {:?}", match self.color {
            Color::Blue => "blue",
            Color::Red => "red",
            Color::Orange => "orange"
        }, self.dimensions, self.weight)
    }
}

fn main() {
    let my_box = Box::create(Color::Orange, (6,9), 32);
    my_box.print()
}
