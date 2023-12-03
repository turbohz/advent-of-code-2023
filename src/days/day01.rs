// https://adventofcode.com/2023/day/1

use std::str;

pub fn solve(input: &str) -> String {
	input
		.lines()
		.map(|l| {
			// collect digits
			let digits:Vec<&str> = l.matches(char::is_numeric).collect();
			// combine first+last digits into a u32
			if let (Some(f),Some(l)) = (digits.first(),digits.last()) {
				format!("{f}{l}").parse::<u32>().unwrap()
			} else {
				panic!("Expected at least one digit in the line")
			}
		})
		// sum them all
		.sum::<u32>()
		.to_string()
}

#[cfg(test)]
mod tests {
	use super::solve;
	use pretty_assertions::assert_str_eq;
	const EXPECTED : &str = "142";
	const INPUT : &str =
r###"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"###;

	#[test]
	fn test() {
		let actual = solve(INPUT);
		assert_str_eq!(actual, EXPECTED);
	}
}
