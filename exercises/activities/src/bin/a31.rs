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
    fn cost(&self) -> f64;
}

struct Carpet(f64);
struct Tile(f64);
struct Wood(f64);

impl Material for Carpet {
    fn cost(&self) -> f64 {
        self.0 * 10.0
    }
}

impl Material for Tile {
    fn cost(&self) -> f64 {
        self.0 * 15.0
    }
}

impl Material for Wood {
    fn cost(&self) -> f64 {
        self.0 * 20.0
    }
}

fn total(materials: Vec<Box<dyn Material>>) -> f64 {
    let mut total = 0.0;
    for m in materials {
        total += m.cost();
    }

    total
}

fn main() {
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Carpet(5.0)),
        Box::new(Tile(12.0)),
        Box::new(Wood(1.0)),
    ];

    println!("Total: {:?}", total(materials));
}
