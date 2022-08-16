// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color

// * Use an enum for the box color
enum Color {
	Brown,
	Red,
}

impl Color {
	fn print(&self) {
		match self {
			Color::Brown => println!("color: brown"),
			Color::Red => println!("color: red"),
		}
	}
}

struct Dimensions {
	width: f64,
	height: f64,
	depth: f64,
}

impl Dimensions {
	fn print(&self) {
		println!("width: {:?}", self.width);
		println!("height: {:?}", self.height);
		println!("depth: {:?}", self.depth);
	}
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
	dimensions: Dimensions,
	weight: f64,
	color: Color,
}

impl ShippingBox {
	// * Implement functionality on the box struct to create a new box
	fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
		Self {
			dimensions,
			weight,
			color,
		}
	}

	// * Implement functionality on the box struct to print the characteristics
	fn print(&self) {
		println!("weight: {:?}", self.weight);
		self.dimensions.print();
		self.color.print();
	}
}

fn main() {
	let small_dimensions = Dimensions {
		width: 1.0,
		height: 2.0,
		depth: 3.0,
	};
	let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
	small_box.print();

	let large_dimensions = Dimensions {
		width: 4.20,
		height: 6.9,
		depth: 13.37,
	};
	let large_box = ShippingBox::new(69.0, Color::Brown, large_dimensions);
	large_box.print();
}
