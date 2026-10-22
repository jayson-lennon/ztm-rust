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

struct Box {
    dimentions: i32,
    weight: f64,
    color: Colors,
}

impl Box {
    fn new_box(dimentions: i32, weight: f64, color: Colors) -> Self {
        Self {
            dimentions: dimentions,
            weight: weight,
            color: color,
        }
    }

    fn display_box(&self) {
        println!(
            "Box dim {0}, weight {1} and color {2:?}",
            self.dimentions, self.weight, self.color
        );
    }
}
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

fn main() {
    let boxx: Box = Box::new_box(3, 5.3, Colors::Blue);
    boxx.display_box();
}
