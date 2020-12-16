use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Instruction {
	op: String,
	val: i32,
	count: i32,
}

fn parse(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|f| {
			let inst = f.split_ascii_whitespace().collect::<Vec<_>>();
			Instruction {
				op: inst[0].to_string(),
				val: inst[1].parse::<i32>().unwrap(),
				count: 0,
			}
		})
		.collect::<Vec<_>>()
}

fn run(instructions: &mut Vec<Instruction>) -> (i32, bool) {
	let mut acc = 0;
	let mut ip: i32 = 0;
	reset(instructions);

	loop {
		// Finished program
		if ip >= instructions.len() as i32 {
			return (acc, false);
		}

		// Index going out of bounds, faulty
		if ip < 0 {
			return (acc, true);
		}

		let inst = &mut instructions[ip as usize];
		inst.count += 1;

		// Getting the same instruction twice, faulty
		if inst.count > 1 {
			return (acc, true);
		}

		match &inst.op[..] {
			"acc" => {
				acc += inst.val;
				ip += 1;
			}
			"jmp" => {
				ip += inst.val;
			}
			"nop" => ip += 1,
			_ => {}
		}
	}
}

fn reset(instructions: &mut Vec<Instruction>) {
	for ins in instructions {
		ins.count = 0;
	}
}

fn run_to_end(instructions: &mut Vec<Instruction>) -> i32 {
	for i in 0..instructions.len() {
		let mut instruction = &mut instructions[i];
		match instruction.op.as_str() {
			"jmp" => {
				instruction.op = "nop".to_string();
			}
			"nop" => {
				instruction.op = "jmp".to_string();
			}
			_ => {}
		}
		let (acc, faulty) = run(instructions);

		if !faulty {
			return acc;
		}

		let mut instruction = &mut instructions[i];
		match instruction.op.as_str() {
			"jmp" => {
				instruction.op = "nop".to_string();
			}
			"nop" => {
				instruction.op = "jmp".to_string();
			}
			_ => {}
		}
	}
	-1
}

#[test]
fn test() {
	let input = "\
		nop +0
		acc +1
		jmp +4
		acc +3
		jmp -3
		acc -99
		acc +1
		jmp -4
		acc +6";
	let mut instructions = parse(input);
	assert_eq!(5, run(&mut instructions).0);
}

#[test]
fn test2() {
	let input = "\
		nop +0
		acc +1
		jmp +4
		acc +3
		jmp -3
		acc -99
		acc +1
		jmp -4
		acc +6";
	let mut instructions = parse(input);
	assert_eq!(8, run_to_end(&mut instructions));
}

fn main() {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

	let mut instructions = parse(&buffer);
	println!("{:?}", run(&mut instructions));

	// Part 2
	println!("{:?}", run_to_end(&mut instructions));
}
