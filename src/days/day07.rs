// https://adventofcode.com/2023/day/7

use std::str;

mod card {

	static GRADE:[char;13] = ['A','K','Q','J','T','9','8','7','6','5','4','3','2'];

	#[derive(Debug, PartialEq, Eq)]
	pub struct Card {
		pub value: char,
		pub grade: u8,
	}

	impl Ord for Card {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			self.grade.cmp(&other.grade)
		}
	}

	impl PartialOrd for Card {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl From<char> for Card {
		fn from(c:char) -> Self {
			let grade = GRADE.into_iter().position(|x|x==c).unwrap().try_into().unwrap();
			Card { grade, value: c.to_owned() }
		}
	}

	impl TryFrom<usize> for Card {

		type Error = ();

		fn try_from(r:usize) -> Result<Self,Self::Error> {
			match r {
				0..=12 => {
					let grade:u8 = r.try_into().unwrap();
					let value = GRADE[r];
					Ok(Card { value, grade })
				}
				_ => Err(())
			}
		}
	}
}

mod hand {

	use super::card::Card;

	#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
	pub enum Category {
		FiveOfAKind,
		FourOfAKind,
		FullHouse,
		ThreeOfAKind,
		TwoPair,
		OnePair,
		HighCard,
	}

	#[derive(Debug,PartialEq,Eq)]
	pub struct Hand {
		pub bid: u32,
		cards: [Card;5],
		category: Category,
	}

	impl Ord for Hand {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			// If categories are different, use them
			match self.category.cmp(&other.category) {
				core::cmp::Ordering::Equal => {}
				ord => return ord,
			}
			// Otherwise, check actual card grades
			for (a,b) in self.cards.iter().zip(&other.cards) {
				match a.cmp(b) {
					core::cmp::Ordering::Equal => {},
					ord => {
						return ord
					},
				}
			}
			// ... all cards equal?
			core::cmp::Ordering::Equal
		}
	}

	impl PartialOrd for Hand {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl Hand {

		pub fn new(line:&str) -> Self {
			let mut parts = line.split_whitespace();
			let cards:[Card;5] = parts.next().unwrap().chars().map(Card::from).collect::<Vec<Card>>().try_into().unwrap();
			let bid:u32   = parts.next().and_then(|s| s.parse().ok()).unwrap();
			Hand { category: Hand::grade(&cards), bid, cards }
		}

		pub fn grade(cards:&[Card;5]) -> Category {
			let mut grouped = cards.iter().fold([0;13],|mut buckets:[usize;13],card| {
				let grade:usize = card.grade.into();
				buckets[grade] += 1;
				buckets
			})
			.into_iter()
			.filter(|c| c > &0usize)
			.collect::<Vec<usize>>();

			grouped.sort();
			grouped.reverse();

			// We now have the counts of equal cards, from higher to lower

			match grouped.len() {
				1 => Category::FiveOfAKind,
				2 => match grouped.first().unwrap() {
						4 => Category::FourOfAKind,
						3 => Category::FullHouse,
						_ => unreachable!()
				},
				3 => if grouped.iter().any(|c| c == &3usize) {
					Category::ThreeOfAKind
				} else {
					Category::TwoPair
				},
				4 => Category::OnePair,
				5 => Category::HighCard,
				_ => unreachable!()
			}
		}
	}
}

use hand::Hand;

pub fn solve(input: &str) -> String {

	let mut hands:Vec<Hand> = input.lines().map(Hand::new).collect();
	hands.sort();
	hands.reverse();

	let total = hands.into_iter().enumerate().map(|(i,h)| {
		let rank:u64 = (i+1).try_into().unwrap();
		let bid:u64 = h.bid.into();
		bid.checked_mul(rank).unwrap()
	}).sum::<u64>();

	total.to_string()
}

#[cfg(test)]
mod tests {
	use super::solve;
	use pretty_assertions::assert_str_eq;

	#[test]
	fn test_solve() {
		let input : &str =
r###"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"###.trim_start();
		let expected : &str = "6440";
		let actual = solve(input);
		assert_str_eq!(actual, expected);
	}
}
