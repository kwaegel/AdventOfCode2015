 
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
	x: i32,
	y: i32,
}

fn read_file(filename: String) -> Result<String, io::Error> {
	let mut f = try!(File::open(filename));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	return Ok(s)
}
 
fn main() {
 
	let input = read_file("day3.txt".to_string());
	assert!(input.is_ok());
	let input_string = input.unwrap();
	let chars = input_string.chars();
	
	let mut visited = HashMap::new();
	visited.insert(Point{x:0,y:0}, 0);
	
	let mut active_point = Point{x:0, y:0};
	let mut inactive_point = Point{x:0, y:0};
	
	
	for c in chars {
		match c {
			'^' => active_point.y += 1,
			'v' => active_point.y -= 1,
			'<' => active_point.x -= 1,
			'>' => active_point.x += 1,
			_ => { /* ignore everything else */ }
		}
		
		// Get the count at the current point, inserting empty value if needed
		// and incrementing it.
		let key = Point{.. active_point};
		*visited.entry(key).or_insert(0) += 1;
		
		std::mem::swap(&mut active_point, &mut inactive_point);
		
	}
	
	let mut num_visited = 0;
	for (_,_) in &visited {
		num_visited += 1;
	}
	
	println!("Number of houses visited: {}", num_visited);
}