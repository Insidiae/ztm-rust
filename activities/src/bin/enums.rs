// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal

// * Use an enum with color names as variants
enum Color {
	Red,
	Yellow,
	Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(my_color: Color) {
	// * Use a match expression to determine which color name to print
	match my_color {
		Color::Red => println!("red"),
		Color::Yellow => println!("yellow"),
		Color::Blue => println!("blue"),
	}
}

fn main() {
	print_color(Color::Red);
	print_color(Color::Yellow);
	print_color(Color::Blue);
}
