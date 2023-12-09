// https://adventofcode.com/2023/day/6

#[derive(PartialEq,Debug)]
struct BestRace {
	time: u32,
	dist: u32,
}

impl From<(u32,u32)> for BestRace {
	fn from((time,dist): (u32,u32)) -> Self {
		BestRace { time, dist }
	}
}

pub fn solve(input: &str) -> String {

	let mut row_values = input.lines().map(|line| {
		let raw_values = line.split(':').last().unwrap();
		raw_values
			.split_whitespace()
			.map(str::parse::<u32>)
			.filter_map(Result::ok)
			.collect::<Vec<u32>>()
	});

	let times = row_values.next().unwrap();
	let distances = row_values.next().unwrap();

	let best_races =
		times
		.into_iter()
		.zip(distances)
		.map(BestRace::from)
		.collect::<Vec<BestRace>>();

	best_races
		.into_iter()
		.map(| BestRace { time: time_limit, dist: distance_record } |

			(0..=time_limit.to_owned())
			.map(|time_pressed| {
				// compute distance travelled
				let speed = time_pressed;
				let travel_time = time_limit.checked_sub(time_pressed).unwrap();
				let distance_travelled = speed * travel_time;
				(time_pressed, distance_travelled)
			})
			.filter(|(_,d)| d > &distance_record)
			.count()

		).
		product::<usize>()
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
Time:      7  15   30
Distance:  9  40  200
"###.trim_start();
			let expected : &str = "288";
			let actual = solve(input);
			assert_str_eq!(actual, expected);
		}
}
