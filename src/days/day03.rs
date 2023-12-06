// https://adventofcode.com/2023/day/3

#[derive(Debug,Clone)]
struct Vec2 {
	x:u32,
	y:u32,
}

impl Vec2 {
	pub fn distance_to(&self,v:&Vec2) -> Distance {
		Distance
			{ x: self.x.abs_diff(v.x)
			, y: self.y.abs_diff(v.y)
			}
	}
}

type Position = Vec2;
type Distance = Vec2;

#[derive(Debug,Clone)]
enum Value {
	Digit(u32),
	Symbol,
	Void,
}

#[derive(Debug,Clone)]
struct Cell {
	pos: Position,
	val: Value
}
impl Cell {
	pub fn new(pos:Position,chr:char) -> Self {
		Cell {
			pos,
			val: match chr {
				'.'  => Value::Void,
				chr  if chr.is_ascii_digit() => Value::Digit(chr.to_digit(10).unwrap()),
				_chr => Value::Symbol
			}
		}
	}
	pub fn is_adjacent_to(&self, c:&Cell) -> bool {
		let d = self.pos.distance_to(&c.pos);
		std::cmp::max(d.x,d.y) <= 1
	}
}

#[derive(Debug,Clone)]
struct PartNo {
	val: u32,
	cells: Vec<Cell>,
}
impl PartNo {
	pub fn new(cell:Cell) -> Self {
		let Value::Digit(val) = cell.val else { panic!() };
		PartNo { val, cells: vec![cell] }
	}
	pub fn grow(&self, cell:Cell) -> Self {
		let Value::Digit(val) = cell.val else { panic!() };
		PartNo {
			val : self.val*10 + val,
			cells: [self.cells.clone(),vec![cell]].concat()
		}
	}
}

fn parse(input: &str) -> (Vec<PartNo>,Vec<Cell>) {

	let mut partnos:Vec<PartNo> = vec![];
	let mut symbols:Vec<Cell>   = vec![];
	let eol = || std::iter::once('.');

	input
		.lines()
		.enumerate()
		.for_each(|(i,row)| {

			// Build a part number as we go

			let mut partno_partial:Option<PartNo> = None;

			row
			.chars()
			.chain(eol()) // add terminator value, to make trailing 'partnos' enclosed
			.enumerate()
			.for_each(|(j,c)| {
				let y = u32::try_from(i).unwrap();
				let x = u32::try_from(j).unwrap();
				let cell = Cell::new(Vec2 { x, y }, c);

				if matches!(cell.val,Value::Digit(_)){

					if let Some(partno) = &partno_partial {
						partno_partial = Some(partno.grow(cell));
					} else {
						partno_partial = Some(PartNo::new(cell));
					}

				} else {

					if matches!(cell.val,Value::Symbol) {
						symbols.push(cell.clone());
					}

					if let Some(partno) = &partno_partial {
						partnos.push(partno.clone());
						partno_partial = None;
					}
				}
			});
		});

	(partnos,symbols)
}

pub fn solve(input:&str) -> String {

	let (partnos,symbols) = parse(input);

	partnos
		.iter()
		.filter(|pn| {
			pn.cells.iter()
			.any(|p| symbols.iter().any(|s| s.is_adjacent_to(p)))
		})
		.map(|pn| pn.val )
		.sum::<u32>()
		.to_string()
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_str_eq;
	use super::solve;

	#[test]
	fn test_solve() {
		let input : &str =
r###"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"###.trim_start();
		let expected : &str = "4361";
		let actual = solve(input);
		assert_str_eq!(actual, expected);
	}
}
