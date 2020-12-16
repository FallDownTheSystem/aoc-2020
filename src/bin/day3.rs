use std::io;
use std::io::prelude::*;

fn trees_hit(slope: &[&[u8]], step_x: i64, step_y: i64) -> i64 {
	let mut hit = 0;
	let mut x = 0;
	let mut y = 0;
	let goal = slope.len() as i64;
	let width = slope[0].len() as i64;
	while y < goal - 1 {
		x = (x + step_x) % width;
		y += step_y;
		let pos = slope[y as usize][x as usize];
		if pos == b'#' {
			hit += 1;
		}
	}
	hit
}

#[test]
fn test1() {
	let input = "\
		..##.......
		#...#...#..
		.#....#..#.
		..#.#...#.#
		.#...##..#.
		..#.##.....
		.#.#.#....#
		.#........#
		#.##...#...
		#...##....#
		.#..#...#.#";

	let lines = input.lines().map(|f| f.trim().as_bytes()).collect::<Vec<_>>();
	assert_eq!(7, trees_hit(&lines, 3, 1));
}

#[test]
fn test2() {
	let input = "\
		..##.......
		#...#...#..
		.#....#..#.
		..#.#...#.#
		.#...##..#.
		..#.##.....
		.#.#.#....#
		.#........#
		#.##...#...
		#...##....#
		.#..#...#.#";

	let lines = input.lines().map(|f| f.trim().as_bytes()).collect::<Vec<_>>();
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let mut results = Vec::<i64>::new();
	for &slope in &slopes {
		results.push(trees_hit(&lines, slope.0, slope.1))
	}
	assert_eq!(336, results.iter().product::<i64>());
}

fn main() {
	let mut buf = String::new();
	io::stdin().read_to_string(&mut buf).unwrap();
	let lines = buf.lines().map(|f| f.trim().as_bytes()).collect::<Vec<_>>();

	// Part 1
	let hits = trees_hit(&lines, 3, 1);
	println!("{}", hits);

	// Part 2
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	// Using i64 since i32 ran out of space.
	let mut results = Vec::<i64>::new();
	for &slope in &slopes {
		results.push(trees_hit(&lines, slope.0, slope.1))
	}
	let hits = results.iter().product::<i64>();
	println!("{}", hits);
}
