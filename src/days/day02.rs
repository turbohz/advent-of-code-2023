// https://adventofcode.com/2023/day/2

mod hand {

	#[derive(Default,PartialEq,Debug)]
	pub struct Hand {
		pub red   :u32,
		pub green :u32,
		pub blue  :u32,
	}

	impl Hand {
		pub fn new(input:&str) -> Self {
			// parse "<part>, <part>, (...)"
			input
			.split(", ")
			.map(|c| {
				// parse "<n> <color>"
				let mut parts = c.split_whitespace();
				let amount = parts.next();
				let color  = parts.next();
				if let (Some(left),Some(right)) = (amount,color) {
					let n = left.parse::<u32>().unwrap();
					match right {
						"red"   => Hand { red:   n, ..Default::default() },
						"green" => Hand { green: n, ..Default::default() },
						"blue"  => Hand { blue:  n, ..Default::default() },
						_ => panic!("Invalid color")
					}
				} else {
					panic!("Invalid \"<n> <color>\" pair")
				}
			})
			// combine the partial hands, by adding them
			.fold(Hand::default(),Hand::add)
		}
		pub fn add(self,other:Hand) -> Hand {
			Hand {
				red: self.red + other.red,
				green: self.green + other.green,
				blue: self.blue + other.blue,
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use pretty_assertions::assert_eq;
		use super::*;

		#[test]
		fn test_parse_hand() {
			const INPUT : &str = "8 green, 6 blue, 20 red";
			const EXPECTED : Hand = Hand { red: 20, green: 8, blue:6 };
			let actual = Hand::new(INPUT);
			assert_eq!(actual, EXPECTED);
		}
	}
}

mod game {

	use super::hand::*;
	use std::result::Result;

	#[derive(PartialEq,Debug)]
	pub struct Game {
		pub id: u32,
		pub hands: Vec<Hand>,
	}

	impl Game {
		pub fn new(input:&str) -> Game {

			let mut parts = input.split(": ");
			let header = parts.next().unwrap();
			let body  = parts.next().unwrap();

			// parse "Game <n>"
			let id:u32 = header
				.split_whitespace().last()
				.map(str::parse::<u32>)
				.and_then(Result::ok).unwrap();

			// parse list of hands
			let hands:Vec<Hand> = body.split("; ").map(Hand::new).collect();
			Game { id, hands }
		}
	}

	#[cfg(test)]
	mod tests {
		use pretty_assertions::assert_eq;
		use super::*;

		#[test]
		fn test_parse_game() {
			const INPUT : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
			let expected : Game =
				Game {
					id: 1,
					hands: vec![
						Hand { red: 4, green: 0, blue: 3 },
						Hand { red: 1, green: 2, blue: 6 },
						Hand { red: 0, green: 2, blue: 0 },
					] };
			let actual = Game::new(INPUT);
			assert_eq!(actual, expected);
		}
	}
}

use game::Game;

pub fn solve(input: &str) -> String {

	// no hand should have more than this amount of any colored cubes
	const MAX_RED:u32 = 12;
	const MAX_GREEN:u32 = 13;
	const MAX_BLUE:u32 = 14;

	input
	.lines()
	.map(Game::new)
	.filter(|game| {
		// all hands should have no more colored cubes than the max asserted
		game.hands
		.iter()
		.all(|hand|
			hand.red   <= MAX_RED &&
			hand.green <= MAX_GREEN &&
			hand.blue  <= MAX_BLUE
		)
	})
	.map(|game| game.id)
	.sum::<u32>()
	.to_string()
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_str_eq;
	use super::solve;

	#[test]
		fn test_solve() {
			const INPUT : &str =
r###"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"###;
			const EXPECTED : &str = "8";
			let actual = solve(INPUT);
			assert_str_eq!(actual, EXPECTED);
		}
}
