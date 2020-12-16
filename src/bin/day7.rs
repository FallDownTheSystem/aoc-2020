use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn parse_bags(lines: &[&str]) -> HashMap<String, HashMap<String, i64>> {
	let mut rules: HashMap<String, HashMap<String, i64>> = HashMap::new();
	for &line in lines.iter() {
		let re = Regex::new(r"(?P<qty>\d) (?P<color>\w+ \w+)").unwrap();
		let rule = line
			.split(" bags contain ")
			.collect::<Vec<_>>()
			.first()
			.unwrap()
			.to_string();
		let captures = re.captures_iter(line);
		let mut children = HashMap::<String, i64>::new();
		for capture in captures {
			let color = capture.name("color").unwrap().as_str().to_string();
			let quantity = capture.name("qty").unwrap().as_str().parse::<i64>().unwrap();
			children.insert(color, quantity);
		}
		rules.insert(rule, children);
	}
	rules
}

// Part 1
fn check_bags_for_color(
	bags: &HashMap<String, HashMap<String, i64>>,
	bag: &HashMap<String, i64>,
	search_color: &str,
) -> bool {
	if bag.is_empty() {
		return false;
	}

	for child in bag {
		let color = child.0;
		if search_color == color {
			return true;
		} else {
			let children = bags.get(color).unwrap();
			let found = check_bags_for_color(bags, children, search_color);
			if found {
				return true;
			}
		}
	}
	false
}

fn count_bags(bags: &HashMap<String, HashMap<String, i64>>, color: &str) -> i64 {
	let mut counter = 0;
	for rule in bags {
		let children = rule.1;
		if check_bags_for_color(&bags, &children, color) {
			counter += 1
		};
	}
	counter
}

#[test]
fn test1() {
	let input = "\
	light red bags contain 1 bright white bag, 2 muted yellow bags.
	dark orange bags contain 3 bright white bags, 4 muted yellow bags.
	bright white bags contain 1 shiny gold bag.
	muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
	shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
	dark olive bags contain 3 faded blue bags, 4 dotted black bags.
	vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
	faded blue bags contain no other bags.
	dotted black bags contain no other bags.";

	let lines = input.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let bags = parse_bags(&lines);
	let count = count_bags(&bags, "shiny gold");
	assert_eq!(4, count);
}

// Part 2
fn count_nested_bags(bags: &HashMap<String, HashMap<String, i64>>, search_color: &str) -> i64 {
	let mut total = 0;
	let children = bags.get(search_color).unwrap();

	if children.is_empty() {
		return 0;
	}
	for (color, qty) in children {
		total += qty * count_nested_bags(bags, color) + qty;
	}
	total
}

#[test]
fn test2() {
	let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
	dark orange bags contain 3 bright white bags, 4 muted yellow bags.
	bright white bags contain 1 shiny gold bag.
	muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
	shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
	dark olive bags contain 3 faded blue bags, 4 dotted black bags.
	vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
	faded blue bags contain no other bags.
	dotted black bags contain no other bags.";

	let lines = input.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let bags = parse_bags(&lines);
	let total = count_nested_bags(&bags, "shiny gold");

	assert_eq!(32, total);
}

#[test]
fn test3() {
	let input = "shiny gold bags contain 2 dark red bags.
	dark red bags contain 2 dark orange bags.
	dark orange bags contain 2 dark yellow bags.
	dark yellow bags contain 2 dark green bags.
	dark green bags contain 2 dark blue bags.
	dark blue bags contain 2 dark violet bags.
	dark violet bags contain no other bags.";
	let lines = input.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let bags = parse_bags(&lines);

	let total = count_nested_bags(&bags, "shiny gold");
	assert_eq!(126, total);
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

	let lines = buffer.lines().map(|f| f.trim()).collect::<Vec<_>>();
	let bags = parse_bags(&lines);
	let count = count_bags(&bags, "shiny gold");
	println!("{}", count);

	let count = count_nested_bags(&bags, "shiny gold");
	println!("{}", count);
}
