// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock

use std::collections::HashMap;

fn main() {
	// * Use a HashMap for the furniture store stock
	let mut furniture_stock = HashMap::new();
	furniture_stock.insert("Chair", 5);
	furniture_stock.insert("Bed", 3);
	furniture_stock.insert("Table", 2);
	furniture_stock.insert("Couch", 0);

	let mut total_items = 0;

	for (item, quantity) in furniture_stock.iter() {
		total_items += quantity;

		let stock_count = if quantity == &0 {
			String::from("Out of stock")
		} else {
			format!("{:?}", quantity)
		};
		println!("Item: {:?}, Stock: {:?}", item, stock_count);
	}

	println!("Total items in stock: {:?}", total_items);
}
