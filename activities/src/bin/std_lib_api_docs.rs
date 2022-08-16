// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase

fn main() {
	let my_str = "this is my STRING";

	// * Utilize standard library functionality to
	//   transform the string to lowercase and uppercase
	// * Use 'rustup doc' in a terminal to open the standard library docs
	// * Navigate to the API documentation section
	// * Search for functionality to transform a string (or str)
	//   to uppercase and lowercase
	//   * Try searching for: to_uppercase, to_lowercase
	println!("UPPERCASE: {:?}", my_str.to_uppercase());
	println!("lowercase: {:?}", my_str.to_lowercase());
}
