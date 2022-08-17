// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
use std::io;
use std::io::Write;

// * Use an enum to store the possible power states
enum PowerState {
	Off,
	Sleep,
	Reboot,
	Shutdown,
	Hibernate,
}

impl PowerState {
	fn new(input: &str) -> Option<PowerState> {
		// * The program should be case-insensitive (the user should be able to type
		//   Reboot, reboot, REBOOT, etc.)
		let state = input.trim().to_lowercase();

		// * Use a match expression to convert the user input into the power state enum
		match state.as_str() {
			"off" => Some(PowerState::Off),
			"sleep" => Some(PowerState::Sleep),
			"reboot" => Some(PowerState::Reboot),
			"shutdown" => Some(PowerState::Shutdown),
			"hibernate" => Some(PowerState::Hibernate),
			_ => None,
		}
	}
}

// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn print_power_action(power_state: PowerState) {
	use PowerState::*;
	match power_state {
		Off => println!("Turning off..."),
		Sleep => println!("Sleeping..."),
		Reboot => println!("Rebooting..."),
		Shutdown => println!("Shutting down..."),
		Hibernate => println!("Hibernating..."),
	}
}

fn main() {
	let mut buffer = String::new();
	print!("Enter new power state: ");
	//? Display prompt message before reading input
	//? https://stackoverflow.com/a/34993992
	io::stdout().flush().unwrap();
	let user_input_status = io::stdin().read_line(&mut buffer);

	if user_input_status.is_ok() {
		match PowerState::new(&buffer) {
			Some(power_state) => print_power_action(power_state),
			None => println!("Invalid power state"),
		}
	} else {
		println!("Error reading input");
	}
}
