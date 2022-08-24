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

trait Body {}
// * Examples of car bodies can be Truck, Car, Scooter
#[derive(Debug)]
struct Truck;
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors
impl Body for Truck {}
#[derive(Debug)]
struct Car;
impl Body for Car {}
#[derive(Debug)]
struct Scooter;
impl Body for Scooter {}

trait Color {}
// * Examples of colors could be red, white, black
#[derive(Debug)]
struct Red;
impl Color for Red {}
#[derive(Debug)]
struct White;
impl Color for White {}
#[derive(Debug)]
struct Black;
impl Color for Black {}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
	body: B,
	color: C,
}
impl<B: Body, C: Color> Vehicle<B, C> {
	fn new(body: B, color: C) -> Self {
		Self { body, color }
	}
}

fn main() {
	let red_truck = Vehicle::new(Truck, Red);
	let white_car = Vehicle::new(Car, White);
	let black_scooter = Vehicle::new(Scooter, Black);

	println!("{:?}", red_truck);
	println!("{:?}", white_car);
	println!("{:?}", black_scooter);
}
