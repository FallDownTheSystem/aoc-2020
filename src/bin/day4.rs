use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;
#[derive(Default)]
struct Passport {
	birth_year: Option<String>,
	issue_year: Option<String>,
	expiration_year: Option<String>,
	height: Option<String>,
	hair_color: Option<String>,
	eye_color: Option<String>,
	passport_id: Option<String>,
	country_id: Option<String>,
}

impl Passport {
	fn valid(&self) -> bool {
		self.valid_birth_year()
			&& self.valid_issue_year()
			&& self.valid_expiration_year()
			&& self.valid_height()
			&& self.valid_hair_color()
			&& self.valid_eye_color()
			&& self.valid_passport_id()
	}

	fn valid_height(&self) -> bool {
		if let Some(height) = &self.height {
			let len = height.len();
			let value = height[..len - 2].parse::<i32>();
			let unit = &height[len - 2..];
			if let Ok(value) = value {
				match unit {
					"cm" => return value >= 150 && value <= 193,
					"in" => return value >= 59 && value <= 76,
					_ => return false,
				}
			}
			return false;
		}
		false
	}

	fn valid_birth_year(&self) -> bool {
		if let Some(birth_year) = &self.birth_year {
			if let Ok(value) = birth_year.parse::<i32>() {
				return value >= 1920 && value <= 2002;
			}
			return false;
		}
		false
	}

	fn valid_issue_year(&self) -> bool {
		if let Some(issue_year) = &self.issue_year {
			if let Ok(value) = issue_year.parse::<i32>() {
				return value >= 2010 && value <= 2020;
			}
			return false;
		}
		false
	}

	fn valid_expiration_year(&self) -> bool {
		if let Some(expiration_year) = &self.expiration_year {
			if let Ok(value) = expiration_year.parse::<i32>() {
				return value >= 2020 && value <= 2030;
			}
			return false;
		}
		false
	}

	fn valid_hair_color(&self) -> bool {
		if let Some(hair_color) = &self.hair_color {
			let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
			return re.is_match(hair_color);
		}
		false
	}

	fn valid_eye_color(&self) -> bool {
		if let Some(eye_color) = &self.eye_color {
			let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
			return re.is_match(eye_color);
		}
		false
	}

	fn valid_passport_id(&self) -> bool {
		if let Some(passport_id) = &self.passport_id {
			let re = Regex::new(r"^\d{9}$").unwrap();
			return re.is_match(passport_id);
		}
		false
	}
}

impl FromStr for Passport {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let attributes = s
			.split_ascii_whitespace()
			.map(|x| x.trim().split(':').collect::<Vec<_>>())
			.collect::<Vec<_>>();

		let mut passport = Passport::default();
		for attribute in attributes {
			match &attribute[..] {
				["byr", val] => passport.birth_year = Some(val.to_string()),
				["iyr", val] => passport.issue_year = Some(val.to_string()),
				["eyr", val] => passport.expiration_year = Some(val.to_string()),
				["hgt", val] => passport.height = Some(val.to_string()),
				["hcl", val] => passport.hair_color = Some(val.to_string()),
				["ecl", val] => passport.eye_color = Some(val.to_string()),
				["pid", val] => passport.passport_id = Some(val.to_string()),
				["cid", val] => passport.country_id = Some(val.to_string()),
				_ => panic!("Unexpected attribute {:?}", attribute),
			}
		}
		Ok(passport)
	}
}

fn parse_passports(input: &str) -> Vec<Passport> {
	let mut passports = Vec::new();
	for passport_str in input.split("\n\n") {
		passports.push(Passport::from_str(passport_str).unwrap()); // Handle errors
	}

	passports
}

#[test]
fn test1() {
	let input = "\
		ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
		byr:1937 iyr:2017 cid:147 hgt:183cm

		iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
		hcl:#cfa07d byr:1929

		hcl:#ae17e1 iyr:2013
		eyr:2024
		ecl:brn pid:760753108 byr:1931
		hgt:179cm

		hcl:#cfa07d eyr:2025 pid:166559648
		iyr:2011 ecl:brn hgt:59in";

	let passports = parse_passports(input);
	assert_eq!(2, passports.iter().filter(|x| x.valid()).count());
}

fn main() {
	let mut buf = String::new();
	io::stdin().read_to_string(&mut buf).unwrap();

	println!("{}", parse_passports(&buf.trim()).iter().filter(|x| x.valid()).count());
}
