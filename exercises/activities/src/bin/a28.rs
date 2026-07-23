// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type color specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shoe(Color);
impl Shoe {
    fn new(color: Color) -> Self {
        Self(color)
    }

    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}
#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }

    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}
#[derive(Debug)]
struct Pants(Color);
impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }

    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}
fn main() {
    let brown_shirt = Shirt::new(Color::Brown);
    let red_shoe = Shoe::new(Color::Red);
    let mut custom_pants = Pants::new(Color::Custom("Deep blue".to_string()));

    println!("{:?}", brown_shirt);
    println!("{:?}", red_shoe);
    println!("{:?}", custom_pants);
    custom_pants.set_color(Color::Blue);
    println!("{:?}", custom_pants);
}
