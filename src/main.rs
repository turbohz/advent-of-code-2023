use std::{env, error::Error};
use aoc_driver::*;

mod days;
use days::*;

fn main()->Result<(),Box<dyn Error>> {
	let cookie : String = env::var("COOKIE")?;
	aoc_magic!(&cookie, 2023:1:1, day01::solve)?;
	aoc_magic!(&cookie, 2023:2:1, day02::solve)?;
	aoc_magic!(&cookie, 2023:3:1, day03::solve)?;
	aoc_magic!(&cookie, 2023:4:1, day04::solve)?;
	println!("All done!");
	Ok(())
}
