
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;

extern crate regex;
use regex::Regex;

enum Ineq {
	Equals(i32),
	GreaterThan(i32),
	LessThan(i32),
}

// --------------------------------------------------------

fn find_sue (constraints: &HashMap<&str, Ineq>, filename: &str) -> i32 {

	let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
	
    let reader = BufReader::new(file);
    let lines = reader.lines();
	
	let re = Regex::new(r"(?P<key>[:alpha:]+): (?P<value>\d+)").unwrap();
	
	let mut sue_num = 0;
	let mut sue_no_conflict = -1i32;
	for line in lines {
		let text = line.unwrap();
		sue_num += 1;
		
		let mut has_conflict = false;
		for cap in re.captures_iter(&text) {
			let key = cap.name("key").unwrap_or("");
			let value = cap.name("value").unwrap_or("").parse::<i32>().unwrap();
			
			match constraints.get(&key) {
				Some(&Ineq::Equals(present_value)) => {
					if value != present_value {
						has_conflict = true;
					}
				},
				Some(&Ineq::GreaterThan(present_value)) => {
					if value <= present_value {
						has_conflict = true;
					}
				},
				Some(&Ineq::LessThan(present_value)) => {
					if value >= present_value {
						has_conflict = true;
					}
				},
				_ => {},
			}
		}
		
		if !has_conflict {
			println!("Sue {} has no conflicts", sue_num);
			sue_no_conflict = sue_num;
		}
	}
	sue_no_conflict
}


// --------------------------------------------------------

fn main() {

	println!("Running part 1...");
	let mut sue_stats_exact = HashMap::new();
	sue_stats_exact.insert("children", Ineq::Equals(3) );
	sue_stats_exact.insert("cats", Ineq::Equals(7) );
	sue_stats_exact.insert("samoyeds", Ineq::Equals(2) );
	sue_stats_exact.insert("pomeranians", Ineq::Equals(3) );
	sue_stats_exact.insert("akitas", Ineq::Equals(0) );
	sue_stats_exact.insert("vizslas", Ineq::Equals(0) );
	sue_stats_exact.insert("goldfish", Ineq::Equals(5) );
	sue_stats_exact.insert("trees", Ineq::Equals(3) );
	sue_stats_exact.insert("cars", Ineq::Equals(2) );
	sue_stats_exact.insert("perfumes", Ineq::Equals(1) );
	
	find_sue(&sue_stats_exact, "day16.txt");
	
	println!("Running part 2...");
	let mut sue_stats_ineq = HashMap::new();
	sue_stats_ineq.insert("children", Ineq::Equals(3) );
	sue_stats_ineq.insert("cats", Ineq::GreaterThan(7) );
	sue_stats_ineq.insert("samoyeds", Ineq::Equals(2) );
	sue_stats_ineq.insert("pomeranians", Ineq::LessThan(3) );
	sue_stats_ineq.insert("akitas", Ineq::Equals(0) );
	sue_stats_ineq.insert("vizslas", Ineq::Equals(0) );
	sue_stats_ineq.insert("goldfish", Ineq::LessThan(5) );
	sue_stats_ineq.insert("trees", Ineq::GreaterThan(3) );
	sue_stats_ineq.insert("cars", Ineq::Equals(2) );
	sue_stats_ineq.insert("perfumes", Ineq::Equals(1) );
	
	find_sue(&sue_stats_ineq, "day16.txt");
}
