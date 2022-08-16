// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item

// * Use a struct for the grocery item
struct GroceryItem {
	// * Use two i32 fields for the quantity and id number
	id: i32,
	quantity: i32,
}

// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
	println!("id: {:?}", item.id);
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
	println!("quantity: {:?}", item.quantity);
}

fn main() {
	let my_item = GroceryItem {
		id: 69,
		quantity: 420,
	};

	display_id(&my_item);
	display_quantity(&my_item);
}
