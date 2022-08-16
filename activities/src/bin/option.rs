// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students

// * Use a struct containing the student's name and locker assignment
struct Student {
	name: String,
	// * The locker assignment should use an Option<i32>
	locker: Option<i32>,
}

fn main() {
	let student = Student {
		name: String::from("Goku"),
		locker: Some(9001),
	};

	println!("Name: {:?}", student.name);
	match student.locker {
		Some(locker_num) => println!("Assigned Locker Number: #{:?}", locker_num),
		None => println!("No Locker Assigned"),
	}
}
