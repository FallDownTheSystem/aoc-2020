use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
	low: u8,
	high: u8,
	character: char,
	password: String,
}

impl Policy {
	fn valid(&self) -> bool {
		let length = self.password.chars().filter(|c| c == &self.character).count() as u8;
		length >= self.low && length <= self.high
	}

	fn valid_new(&self) -> bool {
		let chars = self.password.chars().collect::<Vec<_>>();
		// I don't like having to map these as usize
		// It's because part 1 they weren't indices, but part 2 they are
		let first = chars[(self.low - 1) as usize] == self.character;
		let second = chars[(self.high - 1) as usize] == self.character;
		(first || second) && first != second
	}
}

// FIX ME! Use FromStr instead
impl From<&str> for Policy {
	fn from(line: &str) -> Self {
		let parts = line.split(' ').collect::<Vec<_>>();
		let policy_limits = parts[0].split('-').map(|f| f.parse().unwrap()).collect::<Vec<_>>();
		// This only works with 'characters' that are a single unicode scalar value
		let policy_character = parts[1].chars().next().unwrap();
		let password = parts[2];

		Policy {
			low: policy_limits[0],
			high: policy_limits[1],
			character: policy_character,
			password: password.to_string(),
		}
	}
}

#[test]
fn test1() {
	// Including tabs because trim() deals with them anyway
	let lines = "\
		1-3 a: abcde
		1-3 b: cdefg
		2-9 c: ccccccccc";
	let lines = lines.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let policies = lines.iter().map(|&x| Policy::from(x)).filter(|x| x.valid()).count();

	assert_eq!(2, policies)
}

#[test]
fn test2() {
	let lines = "\
		1-3 a: abcde
		1-3 b: cdefg
		2-9 c: ccccccccc";
	let lines = lines.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let policies = lines.iter().map(|&x| Policy::from(x)).filter(|x| x.valid_new()).count();

	assert_eq!(1, policies)
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

	let lines = buffer.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let policies = lines.iter().map(|&x| Policy::from(x)).collect::<Vec<_>>();

	let part1_policies = policies.iter().filter(|&x| x.valid()).count();
	println!("{}", part1_policies);

	let part2_policies = policies.iter().filter(|&x| x.valid_new()).count();
	println!("{}", part2_policies);
}
