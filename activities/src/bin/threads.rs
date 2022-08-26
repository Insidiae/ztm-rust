// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"

fn msg_hello() -> &'static str {
	use std::time::Duration;
	std::thread::sleep(Duration::from_millis(1000));
	"Hello, "
}

fn msg_thread() -> &'static str {
	use std::time::Duration;
	std::thread::sleep(Duration::from_millis(1000));
	"threads"
}

fn msg_excited() -> &'static str {
	use std::time::Duration;
	std::thread::sleep(Duration::from_millis(1000));
	"!"
}

fn main() {
	use std::thread;

	let hello_handle = thread::spawn(move || msg_hello());
	let thread_handle = thread::spawn(move || msg_thread());
	let excited_handle = thread::spawn(move || msg_excited());

	// * Use the join function to wait for threads to finish
	let hello = hello_handle.join().expect("failed to join hello");
	let thread = thread_handle.join().expect("failed to join thread");
	let excited = excited_handle.join().expect("failed to join excited");

	println!("{}{}{}", hello, thread, excited);
}
