// https://adventofcode.com/2023/day/8

use std::{str, collections::HashMap};

type Id = [char;3];
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Node(Id);

impl From<&str> for Node {
   fn from(value: &str) -> Self {
		let chars: Vec<char> = value.chars().take(3).collect();
		let id: Id = chars.try_into().unwrap();
		Node(id)
	}
}

pub fn solve(input: &str) -> String {
	let mut lines = input.lines();

	let directions = lines.next().unwrap().chars().cycle();
	let _separator = lines.next();

	// build a HashMap of node -> (left,right)

	let mut paths:HashMap<Node,(Node,Node)> = HashMap::new();

	lines.for_each(|l| {
		let node:Node  = l.get(00..=02).unwrap().into();
		let left:Node  = l.get(07..=09).unwrap().into();
		let right:Node = l.get(12..=14).unwrap().into();
		paths.insert(node,(left,right));
	});

	let mut steps:usize = 0;

	const START:Node = Node(['A','A','A']);
	const END:Node   = Node(['Z','Z','Z']);

	let mut current:Node = START;

	for d in directions {
		let node = &current;

		if *node == END {
			break;
		} else {
			steps += 1;
		}
		let (left,right) = paths.get(node).unwrap();

		match d {
			'L' => current = *left,
			'R' => current = *right,
			_   => unreachable!()
		}
	}

	steps.to_string()
}

#[cfg(test)]
mod tests {
	use super::solve;
	use pretty_assertions::assert_str_eq;

	#[test]
	fn test_solve() {
		let input : &str =
r###"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"###.trim_start();
		let expected : &str = "6";
		let actual = solve(input);
		assert_str_eq!(actual, expected);
	}
}
