use std::io;
use std::io::prelude::*;

const PREAMBLE: usize = 25;

fn find_invalid_number(numbers: &[i64]) -> i64 {
	for i in PREAMBLE..numbers.len() {
		let mut valid = false;
		for j in (i - PREAMBLE)..i {
			for k in (i - PREAMBLE)..i {
				if j != k && numbers[j] + numbers[k] == numbers[i] {
					valid = true;
				}
			}
		}
		if !valid {
			return numbers[i];
		}
	}
	-1
}

fn contiguous_sum(numbers: &[i64], target: i64, start: usize) -> (bool, usize) {
	let mut total = 0;
	for (i, n) in numbers[start..].iter().enumerate() {
		total += n;
		if total == target {
			return (true, i);
		}
		if total > target {
			break;
		}
	}
	(false, 0)
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

	let numbers = buffer
		.lines()
		.map(|f| f.trim().parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	println!("{}", find_invalid_number(&numbers));

	for x in 0..numbers.len() {
		let (matches, end) = contiguous_sum(&numbers, 756008079, x);
		if matches {
			let range = &numbers[x..end];
			let min = range.iter().min().unwrap();
			let max = range.iter().max().unwrap();
			println!("{}", min + max);
			return;
		}
	}
}
