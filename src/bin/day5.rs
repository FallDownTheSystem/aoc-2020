use std::io;
use std::io::prelude::*;

fn parse_seat(input: &str) -> (u16, u16, u16) {
	let binary_string = input
		.replace("B", "1")
		.replace("R", "1")
		.replace("L", "0")
		.replace("F", "0");
	let seat = u16::from_str_radix(&binary_string, 2).unwrap();
	let row = seat / 8;
	let column = seat % 8;
	(row, column, seat)
}
#[test]
fn test1() {
	assert_eq!((70, 7, 567), parse_seat("BFFFBBFRRR"));
	assert_eq!((14, 7, 119), parse_seat("FFFBBBFRRR"));
	assert_eq!((102, 4, 820), parse_seat("BBFFBBFRLL"));
}

fn main() {
	let mut buf = String::new();
	io::stdin().read_to_string(&mut buf).unwrap();
	let mut seats = buf.lines().map(|f| parse_seat(f.trim()).2).collect::<Vec<_>>();

	seats.sort_unstable();

	println!("{}", seats.last().unwrap());

	let mut pos = *seats.first().unwrap();

	for x in seats {
		if x != pos {
			break;
		}
		pos += 1;
	}
	println!("{}", pos)
}
