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

use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
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
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err(String::from("Not allowed color")),
            other => Ok(Self(other)),
        }
    }
}
#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoes_color<'a>(color: ShoesColor) {
    println!("{:?}", color);
}

fn print_shirts_color<'a>(color: ShirtColor) {
    println!("{:?}", color);
}

fn print_pants_color<'a>(color: PantsColor) {
    println!("{:?}", color);
}
fn main() {
    let brown_shirt = ShirtColor::new(Color::Brown);
    match brown_shirt {
        Ok(color) => print_shirts_color(color),
        Err(msg) => println!("{}", msg),
    }
    let red_shoe = ShoesColor::new(Color::Red);
    print_shoes_color(red_shoe);

    let custom_pants = PantsColor::new(Color::Custom("Deep blue".to_string()));
    print_pants_color(custom_pants);

}
