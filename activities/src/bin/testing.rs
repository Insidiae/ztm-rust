// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn check_clamp() {
		let lower = 1;
		let upper = 10;

		let low_n = 0;
		let high_n = 9001;
		let mid_n = 6;

		assert_eq!(clamp(low_n, lower, upper), lower, "Make sure n is >= lower");

		assert_eq!(
			clamp(high_n, lower, upper),
			upper,
			"Make sure n is <= upper"
		);

		assert_eq!(
			clamp(mid_n, lower, upper),
			mid_n,
			"Make sure n stays between lower and upper"
		);
	}

	#[test]
	fn check_div() {
		assert_eq!(
			div(4, 2),
			Some(2),
			"Make sure div returns correct value when a is divisible by b"
		);

		assert_eq!(
			div(7, 4),
			Some(1),
			"Make sure div returns an integer when a is not divisible by b"
		);

		assert_eq!(
			div(3, 0),
			None,
			"Make sure div returns None when attempting to divide by 0"
		)
	}

	#[test]
	fn check_concat() {
		assert_eq!(
			concat("hello", "world"),
			String::from("helloworld"),
			"Make sure concat combines strings witn no spaces in between"
		);

		assert_eq!(
			concat("hello ", "world"),
			String::from("hello world"),
			"Make sure concat does not remove any spaces within each string"
		)
	}
}

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
	if n < lower {
		lower
	} else if n > upper {
		upper
	} else {
		n
	}
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
	match b {
		0 => None,
		_ => Some(a / b),
	}
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
	format!("{}{}", first, second)
}

fn main() {}
