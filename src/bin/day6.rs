use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn unique_answers(input: &str) -> usize {
	input
		.split("\n\n")
		.map(|x| HashSet::<char>::from_iter(x.chars().filter(|&x| !x.is_whitespace())))
		.fold(0, |acc, x| acc + x.len())
}

fn same_answers(input: &str) -> i32 {
	let mut all_answered = 0;

	let groups = input.split("\n\n").collect::<Vec<_>>();
	for group in groups {
		// We need as many answers as there are people for the answer to be counted
		let people = group.lines().count() as i32;
		// Count the number of answers in a group
		let mut answers_counts = HashMap::<char, i32>::new();
		for answer in group.chars().filter(|&x| !x.is_whitespace()) {
			*answers_counts.entry(answer).or_insert(0) += 1;
		}
		all_answered += answers_counts.iter().filter(|x| *x.1 == people).count() as i32;
	}
	all_answered
}

#[test]
fn test() {
	let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

	assert_eq!(11, unique_answers(&input));
	assert_eq!(6, same_answers(&input));
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).unwrap();

	println!("{}", unique_answers(&buffer));
	println!("{}", same_answers(&buffer));
}
