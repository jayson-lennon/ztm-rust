// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

fn total_cost(materials: &Vec<Box<dyn Material>>) -> f64 {
    materials
        .iter()
        .map(|material| material.total_cost())
        .sum()
}

fn main() {
    let carpet = Box::new(Carpet(20.0));
    let tile = Box::new(Tile(10.0));
    let wood = Box::new(Wood(30.0));
    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];

    let total = total_cost(&materials);
    println!("cost = ${}", total);
}
