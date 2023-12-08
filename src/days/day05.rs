// https://adventofcode.com/2023/day/5

mod map {

	use std::{ops::Range, str::Lines, iter};

	#[derive(PartialEq,Debug)]
	enum Step {
		Seed,
		Soil,
		Fertilizer,
		Water,
		Light,
		Temperature,
		Humidity,
		Location
	}

	impl TryFrom<&str> for Step {

		type Error = &'static str;

		fn try_from(s:&str) -> Result<Self,Self::Error> {
			match s {
				"seed"        => Ok(Step::Seed),
				"soil"        => Ok(Step::Soil),
				"fertilizer"  => Ok(Step::Fertilizer),
				"water"       => Ok(Step::Water),
				"light"       => Ok(Step::Light),
				"temperature" => Ok(Step::Temperature),
				"humidity"    => Ok(Step::Humidity),
				"location"    => Ok(Step::Location),
				_             => Err("")
			}
		}
	}

	#[derive(Debug)]
	struct Mapper {
		range: Range<u64>,
		offset: i64,
	}

	impl Default for Mapper {
		fn default() -> Self {
			Mapper { range: 0..u64::MAX, offset: 0 }
		}
	}

	impl From<&str> for Mapper {
		fn from(line:&str) -> Self {
			let mut parts = line.split_whitespace().flat_map(str::parse::<u64>);
			let dst:u64 = parts.next().unwrap();
			let src:u64 = parts.next().unwrap();
			let len:u64 = parts.next().unwrap();

			let start = src;
			let end:u64 = start+len;
			let range = start..end;
			let idst:i64 = dst.try_into().unwrap();
			let isrc:i64 = src.try_into().unwrap();
			let offset:i64 = idst-isrc;

			Mapper { range, offset }
		}
	}

	impl Mapper {

		pub fn map(&self,src:u64) -> Option<u64> {
			if self.range.contains(&src) {
				src.try_into()
				.map(|n:i64| (n + self.offset).try_into())
				.unwrap().ok()
			} else {
				None
			}
		}
	}

	#[derive(Debug)]
	pub struct Map {
		src: Step,
		dst: Step,
		mps: Vec<Mapper>
	}

	impl<'a> TryFrom<&mut Lines<'a>> for Map {

		type Error = ();

		fn try_from(lines:&mut Lines) -> Result<Self,Self::Error> {

			match lines.next() {
				None => Err(()),
				Some(header) => {

					// decode "<src>-to-<dst> map:"

					let (src,dst)= header.split_whitespace().take(1).next()
					.map(|mapping| {
						// taking 3, filter map will ignore the <to>
						let mut parts = mapping.split('-').take(3).filter_map(|p| Step::try_from(p).ok());
						(parts.next().expect(""), parts.next().unwrap())
					}).unwrap();

					// decode mappings

					let mps = lines.take_while(|l| !l.is_empty())
					.map(Mapper::from)
					// add default fallback mapper
					.chain(iter::once_with(Default::default))
					.collect();

					Ok(Map { src, dst, mps })
				}
			}
		}
	}

	impl Map {
		pub fn map(&self,v:u64) -> u64 {
			self.mps.iter().find_map(|m| m.map(v)).unwrap()
		}
	}

	#[cfg(test)]
	mod tests {

		use std::borrow::BorrowMut;

use pretty_assertions::assert_eq;
		use super::*;

		#[test]
		fn test_map() {
			let input : &str =
r###"
seed-to-soil map:
50 98 2
52 50 48
"###.trim_start();
			let expected : &str =
r###"
0     0
1     1
48    48
49    49
50    52
51    53
96    98
97    99
98    50
99    51
"###.trim_start();
			let map = Map::try_from(input.lines().borrow_mut()).unwrap();
			expected.lines().map(str::split_whitespace).for_each(|v| {
				let mut vs = v.flat_map(str::parse::<u64>);
				let src:u64 = vs.next().unwrap();
				let dst:u64 = vs.next().unwrap();
				assert_eq!(map.map(src),dst);
			});
		}
	}
}

use std::borrow::BorrowMut;
use crate::days::day05::map::Map;

pub fn solve(input: &str) -> String {

	let mut lines = input.lines();

	// parse "seeds: <v1> <v2> <v3>"
	let seeds:Vec<u64> = lines.next().unwrap().split(':')
	.nth(1).unwrap().split_whitespace()
	.filter_map(|p| p.parse().ok()).collect();

	// skip empty line
	lines.next();

	let mut maps:Vec<Map> = vec![];

	// parse blocks into maps
	while let Ok(map) = Map::try_from(lines.borrow_mut()) {
		maps.push(map)
	}

	seeds.iter().map(|s|
		maps.iter().fold(*s,|prev,m| m.map(prev))
	)
	.min().unwrap()
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"###.trim_start();
		let expected : &str = "35";
		let actual = solve(input);
		assert_str_eq!(actual, expected);
	}
}
