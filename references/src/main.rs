fn main() {
	let s = String::from("hello");

	change(&s);
}

fn change(some_string: &String) {
	// This will throw an error.
	some_string.push_str(", world");
	// ^^^^^^^^  Cannot borrow mutable.
}
