// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have anybody
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
struct Vehicle<T: Body, U: Color> {
    body: T,
    color: U,
}

#[derive(Debug)]
enum VehicleBody {
    Truck,
    Car,
    Scooter,
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
struct Black;
impl Color for Black {}

impl<T: Body, U: Color> Vehicle<T, U> {
    fn new(body: T, color: U) -> Self {
        Self { body, color }
    }
}

fn main() {
    let benz = Vehicle::new(Truck, Black);
    println!("{:?}", benz);

    let bmw = Vehicle::new(Car, Red);
    println!("{:?}", bmw);
}
