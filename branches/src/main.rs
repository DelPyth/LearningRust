fn main() {
    let condition = true;

	// Will throw error, expected consistant type.
	let number = if condition {
		5
	} else {
		"six"
	};

	println!("The value of number is: {}", number);
}
