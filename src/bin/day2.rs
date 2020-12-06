use std::io;
use std::io::prelude::*;
fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
	// Create a struct
	// - low
	// - high
	// - character
	// - password
	// Parse by splitting on space, then dash on first element
	// Impl a method on the struct, valid(), that takes self
	// write tests
}