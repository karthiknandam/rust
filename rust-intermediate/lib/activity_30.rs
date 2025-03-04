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

use std::io::stdout;

struct VehicleDetails {
    body: String,
    color: String,
    name: String,
}
impl Body for VehicleDetails {
    fn print_car_body(&self) {
        println!("{:?}", self.body);
    }
}
impl Color for VehicleDetails {
    fn print_color(&self) {
        println!("{:?}", self.color)
    }
}
struct Vehicle<T: Body + Color> {
    vehicles: Vec<T>,
}
impl<T> Vehicle<T>
where
    T: Body + Color,
{
    fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }
    fn add(&mut self, t: T) {
        self.vehicles.push(t);
    }
}

trait Body {
    fn print_car_body(&self);
}
trait Color {
    fn print_color(&self);
}

pub fn main_test() {
    let mut vehicles: Vehicle<VehicleDetails> = Vehicle::new();
    vehicles.add(VehicleDetails {
        body: "Range".to_owned(),
        color: "Red".to_owned(),
        name: "BENZ".to_owned(),
    });
    vehicles.add(VehicleDetails {
        body: "Range".to_owned(),
        color: "Red".to_owned(),
        name: "BENZ".to_owned(),
    });
    vehicles.add(VehicleDetails {
        body: "Range".to_owned(),
        color: "Red".to_owned(),
        name: "BENZ".to_owned(),
    });

    for vehicle in vehicles.vehicles.iter() {
        vehicle.print_car_body();
        vehicle.print_color();
    }
}
