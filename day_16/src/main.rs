
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;

extern crate regex;
use regex::Regex;

// --------------------------------------------------------

fn main() {

	let mut my_sue_stats = HashMap::new();
	my_sue_stats.insert("children", 3);
	my_sue_stats.insert("cats", 7);
	my_sue_stats.insert("samoyeds", 2);
	my_sue_stats.insert("pomeranians", 3);
	my_sue_stats.insert("akitas", 0);
	my_sue_stats.insert("vizslas", 0);
	my_sue_stats.insert("goldfish", 5);
	my_sue_stats.insert("trees", 3);
	my_sue_stats.insert("cars", 2);
	my_sue_stats.insert("perfumes", 1);

    let path = Path::new("day16.txt");
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
	for line in lines {
		let text = line.unwrap();
		sue_num += 1;
		
		let mut has_conflict = false;
		for cap in re.captures_iter(&text) {
			let key =cap.name("key").unwrap_or("");
			let value =cap.name("value").unwrap_or("").parse::<i32>().unwrap();
			
			match my_sue_stats.get(&key) {
				Some(&found_value) => {
					if found_value != value {
						has_conflict = true;
					}
				},
				_ => {},
			}
		}
		
		if !has_conflict {
			println!("Sue {} has no conflicts", sue_num);
		}
	}
}
