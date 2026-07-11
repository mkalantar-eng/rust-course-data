// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box

//
// Notes:

// * Use an enum for the box color
enum Color {
    Red,
    Green,
    Blue,
}

// * Use a struct to encapsulate the box characteristics
struct ShippinBox {
    // * Must include dimensions, weight, and color
    dimensions: (u32, u32, u32),
    weight: f64,
    color: Color,
}

impl ShippinBox {
    // * Implement functionality on the box struct to create a new box
    fn new(dimensions: (u32, u32, u32), weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(self: &Self) {
        println!(
            "dimensions: {} x {} x {}",
            self.dimensions.0, self.dimensions.1, self.dimensions.2
        );
        println!("weight: {}", self.weight);

        match self.color {
            Color::Red => println!("color: Red"),
            Color::Green => println!("color: Green"),
            Color::Blue => println!("color: Blue"),
        }
    }
}

fn main() {
    let sb = ShippinBox::new((4, 7, 12), 33.5, Color::Green);
    sb.print();
}
