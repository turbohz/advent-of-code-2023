// https://adventofcode.com/2023/day/4

use std::str;

mod card {

	pub struct Card {
		winn : Vec<u32>,
		hand : Vec<u32>
	}

	impl Card {

		pub fn new(input:&str) -> Self {

			// parse "<winn1> <winn2> (...) | <draw1> <draw2> (...)"

			let mut parts = input
			.split('|')
			.map(|p| {
				p.split_whitespace()
				.filter_map(|n| n.parse::<u32>().ok())
				.collect::<Vec<u32>>()
			});

			let winners = parts.next().expect("There should be a winners list");
			let draw    = parts.next().expect("There should be a draw list");

			Card {
				winn: winners.clone(),
				hand: draw.clone()
			}
		}

		pub fn score(&self) -> u32 {
			let (win,_) : (Vec<u32>,_) =
				self.hand
				.iter()
				.partition(|&d|
					self.winn.iter().any(|w| w==d)
				);

			match win.len() {
				c if c >= 1 => {
					let exp:u32 = c.try_into().unwrap();
					2_u32.pow(exp-1)
				},
				_ => 0
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use pretty_assertions::assert_eq;
		use super::*;

		#[test]
		fn test_score_card() {
			let expected = 4;
			let actual = Card::new("10 27 43 57 | 4 10 14 27 43 55").score();
			assert_eq!(actual, expected);
		}
	}
}

mod game {

	use super::card::Card;

	pub struct Game { pub cards:Vec<Card> }
	impl Game {
		pub fn new(input:&str) -> Self {
			let cards = input
				.lines()
				.map(|l| {
					l.split(':')
					.last()
					.map(Card::new)
					.unwrap()
				})
				.collect();

			Game { cards }
		}
	}
}

use card::Card;
use game::Game;

pub fn solve(input: &str) -> String {

	Game::new(input).cards
	.iter()
	.map(Card::score)
	.sum::<u32>()
	.to_string()
}

#[cfg(test)]
mod tests {
	use super::solve;
	use pretty_assertions::assert_str_eq;

	#[test]
	fn test_solve() {
		let input : &str =
r###"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"###.trim_start();
		let expected : &str = "13";
		let actual = solve(input);
		assert_str_eq!(actual, expected);
	}
}
