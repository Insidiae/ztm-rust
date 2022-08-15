// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5

// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
	(6, 9)
}

fn main() {
	// * Destructure the return value into two variables
	// ? We don't use x in this activity, so prepend it with _
	let (_x, y) = coordinate();

	// * Use an if..else if..else block to determine what to print
	if y > 5 {
		println!("y is greater than 5");
	} else if y < 5 {
		println!("y is less than 5");
	} else {
		println!("y is equal to 5");
	}
}
