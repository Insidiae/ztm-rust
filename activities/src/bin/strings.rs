// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under

// * Use a struct for a persons age, name, and favorite color
struct Person {
	age: i32,
	// * The color and name should be stored as a String
	name: String,
	fav_color: String,
}

// fn print_fav_color(person: &Person) {
// 	println!("{:?}'s favorite color is {:?}", &person.name, &person.color);
// }

fn print(data: &str) {
	println!("{:?}", data);
}

fn main() {
	// * Create and store at least 3 people in a vector
	let people = vec![
		Person {
			age: 7,
			name: "George".to_owned(),
			fav_color: "green".to_owned(),
		},
		Person {
			age: 9,
			name: String::from("Anna"),
			fav_color: String::from("purple"),
		},
		Person {
			age: 14,
			name: "Kaite".to_owned(),
			fav_color: "blue".to_owned(),
		},
	];

	// * Iterate through the vector using a for..in loop
	for person in people {
		// * Use an if expression to determine which person's info should be printed
		if person.age <= 10 {
			// * The name and colors should be printed using a function
			print(&person.name);
			print(&person.fav_color);
		}
	}
}
