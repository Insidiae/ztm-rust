// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color

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

// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
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
	fn new(color: Color) -> Self {
		Self(color)
	}
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
	fn new(color: Color) -> Self {
		Self(color)
	}
}

// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
fn print_shoes_color(color: ShoesColor) {
	println!("Shoes color: {:?}", color)
}

fn print_shirt_color(color: ShirtColor) {
	println!("Shirt color: {:?}", color)
}

fn print_pants_color(color: PantsColor) {
	println!("Pants color: {:?}", color)
}

fn main() {
	let shoes_color = ShoesColor(Color::Gray);
	let shirt_color = ShirtColor(Color::Blue);
	let pants_color = PantsColor(Color::Custom(String::from("Denim")));

	print_shoes_color(shoes_color);
	print_shirt_color(shirt_color);
	print_pants_color(pants_color);
}
