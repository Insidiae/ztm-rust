// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.

fn main() {
	let data = vec![1, 2, 3, 4, 5];

	// * Use an iterator chain to accomplish the task.
	let tripled_over_10: Vec<i32> = data
		.iter()
		.map(|num| num * &3)
		.filter(|num| num > &10)
		.collect();

	for num in tripled_over_10 {
		println!("{:?}", num)
	}
}
