use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

// Part 1
fn find_sum_of_two(sum: i32, integers: &[i32]) -> Option<(i32, i32)> {
	let mut set = HashSet::new();
	for &n in integers {
		let x = sum - n;
		if set.contains(&x) {
			return Some((n, x));
		}
		set.insert(n);
	}
	None
}

#[test]
fn test1() {
	let integers = [1721, 979, 366, 299, 675, 1456];
	let res = find_sum_of_two(2020, &integers).unwrap();
	assert_eq!(res.0 * res.1, 514579);
}

// Part 2
fn find_sum_of_three(sum: i32, integers: &[i32]) -> Option<(i32, i32, i32)> {
	let mut set = HashSet::new();
	for &i in integers {
		for &j in &integers[1..] {
			let x = sum - i - j;
			if set.contains(&x) {
				return Some((i, j, x))
			}
			set.insert(j);
		}
	}
	None
}

#[test]
fn test2() {
	let integers = [1721, 979, 366, 299, 675, 1456];
	let res = find_sum_of_three(2020, &integers).unwrap();
	assert_eq!(res.0 * res.1 * res.2, 241861950);
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
	let integers = buffer.lines().map(|f| f.trim().parse().expect("Failed to parse integer")).collect::<Vec<_>>();
	// Part 1
	let res = find_sum_of_two(2020, &integers);
	if let Some(res) = res {
		println!("{}", res.0 * res.1);
	}
	// Part 2
	let res = find_sum_of_three(2020, &integers);
	if let Some(res) = res {
		println!("{}", res.0 * res.1 * res.2);
	}
}
