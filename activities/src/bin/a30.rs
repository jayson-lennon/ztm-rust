// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    body: B,
    color: C,
}

impl<B, C> Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

#[derive(Debug)]
struct Car;
impl Body for Car {}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Red;
impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

fn main() {
    let red_truck = Vehicle::new(Truck {}, Red {});
    let blue_car = Vehicle::new(Car {}, Blue {});
    println!("{:?}", red_truck);
    println!("{:?}", blue_car);
}
