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
enum VehicleColor {
    Red,
    White,
    Black,
}

impl Body for VehicleBody {}
impl Color for VehicleColor {}

impl Vehicle<VehicleBody, VehicleColor> {
    fn new(body: VehicleBody, color: VehicleColor) -> Self {
        Self { body, color }
    }
}

fn main() {
    let benz = Vehicle::new(VehicleBody::Truck, VehicleColor::White);
    println!("{:?}", benz);

    let bmw = Vehicle::new(VehicleBody::Car, VehicleColor::Black);
    println!("{:?}", bmw);
}
