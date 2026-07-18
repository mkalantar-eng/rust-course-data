// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Shape {
    fn perimeter(&self) -> f64;
}
struct Square {
    side: f64,
}
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}
impl Shape for Square {
    fn perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn print(shape: impl Shape) {
    println!("{}", shape.perimeter());
}

fn main() {
    print(Square { side: 5.0 });
    print(Triangle { a: 5.0, b: 6.0, c: 9.0 });
}
