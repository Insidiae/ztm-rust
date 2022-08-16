// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21

// * Use a struct to store at least the age of a customer
struct Customer {
	age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
fn try_purchase(customer: &Customer) -> Result<(), String> {
	if customer.age < 21 {
		// * The Err variant should detail the reason why they cannot make a purchase
		return Err(String::from("Customer must be at least 21 years old!"));
	}
	// * Return a result from the function
	Ok(())
}

fn main() {
	let ashley = Customer { age: 21 };

	let purchased = try_purchase(&ashley);
	println!("{:?}", purchased);
}
