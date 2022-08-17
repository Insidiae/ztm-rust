// Topic: External crates
//
// Requirements:
// * Display the current date and time

// * Use the `chrono` crate to work with time
use chrono::prelude::*;

fn main() {
	// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
	//   for examples on how to create custom time formats
	let utc_time = Utc::now();
	let local_time = Local::now();

	println!(
		"Current time in UTC: {:?}",
		utc_time.format("%A, %b %e %Y %T").to_string()
	);
	println!(
		"Current time in your local timezone: {:?}",
		local_time.format("%A, %b %e %Y %T").to_string()
	);
}
