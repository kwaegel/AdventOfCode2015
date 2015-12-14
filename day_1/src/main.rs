
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: String) -> Result<String, io::Error> {
	let mut f = try!(File::open(filename));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	return Ok(s)
}

fn main() {
    println!("Hello, world!");
	
	let filename = "day_1_1_input.txt".to_string();
	let input = read_file(filename);
	assert!(input.is_ok());
	let input_string = input.unwrap();
	
	let mut floor = 0;
	let mut step_idx = 0;
	let mut first_basement_step = 0;
	
	for c in input_string.chars() {
		step_idx += 1;
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => { /* ignore everything else */ }
		}
		
		if floor < 0 && first_basement_step == 0 {
			first_basement_step = step_idx
		}
	}
	
	println!("Final floor is {}", floor);
	println!("Entered basement on step {}", first_basement_step);	
}