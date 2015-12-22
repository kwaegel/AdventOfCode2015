use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::cmp;
extern crate permutohedron;

extern crate nalgebra as na;
use na::{DMat};

extern crate regex;
use regex::Regex;

fn get_input_lines(filename: &str) -> Vec<String> {
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

    lines.map(|x| x.unwrap()).collect()
}

// --------------------------------------------------------

fn get_name_index<'a>(name_indices: &mut HashMap<&'a str, usize>, name: &'a str) -> usize {
	let next_open_index = name_indices.len();
	*name_indices.entry(name).or_insert(next_open_index)
}

// --------------------------------------------------------

fn create_happiness_mapping<'a>(strings: &'a Vec<String>) -> (HashMap<&'a str, usize>, DMat<i32>) {

	let mut name_indices: HashMap<&str, usize> = HashMap::new();
	let mut happiness: DMat<i32> = DMat::new_zeros(20, 20);
	
	let re = Regex::new(r"^(?P<name>[:alpha:]+) .* (?P<sign>gain|lose) (?P<happ>\d+) .* (?P<neighbor>[:alpha:]+)\.$").unwrap();

	for string in strings {
		let cap = re.captures(string).unwrap();
		let person_name = cap.name("name").unwrap_or("");
		let gain = cap.name("sign").unwrap_or("");
		let is_gain = gain == "gain";
		let neighbor_name = cap.name("neighbor").unwrap_or("");
		let happiness_value = cap.name("happ").unwrap_or("").parse::<i32>().unwrap();
		
		let happiness_change = if is_gain { happiness_value } else { -happiness_value };
		let person_idx = get_name_index(&mut name_indices, person_name);
		let neighbor_idx = get_name_index(&mut name_indices, neighbor_name);

		happiness[(person_idx, neighbor_idx)] = happiness_change;
		// println!("{: <6} ({}) => {: <6} ({}) = {}", 
			// &person_name, person_idx, &neighbor_name, neighbor_idx, happiness_change);
	};

	(name_indices, happiness)
}

// --------------------------------------------------------

fn score_arrangement(happiness: &DMat<i32>, arrangement: &Vec<usize>) -> i32 {
	let mut score_sum = 0;
	for idx in arrangement {
		let low_idx = if *idx == 0 { arrangement.len()-1 } else { *idx - 1 };
		let high_idx = if *idx == arrangement.len()-1 { 0 } else { *idx + 1 };
		
		let person = arrangement[*idx];		
		let low_neighbor = arrangement[low_idx];
		let high_neighbor = arrangement[high_idx];
		
		let lower_score = happiness[(person, low_neighbor)];
		let upper_score = happiness[(person, high_neighbor)];
		score_sum += lower_score + upper_score;
		
		// println!("  [{}, {}, {}] = {} + {}", 
			// low_neighbor, person, high_neighbor,
			// lower_score, upper_score);
	}
	score_sum
}

// --------------------------------------------------------

fn main() {
	let strings = get_input_lines("day13.txt");
	let (name_indices_map, happiness) = create_happiness_mapping(&strings);
	
	let num_people = name_indices_map.len();
	let mut seating: Vec<usize> = (0..num_people).collect();
	
	let seating_permutations = permutohedron::Heap::new(&mut seating);
	
	let mut max_score = i32::min_value();
	for arrangement in seating_permutations {
		let score = score_arrangement(&happiness, &arrangement);
		max_score = cmp::max(max_score, score);
		//println!("{:?} => {}", arrangement, score);
	}
	
	// Not clear why this won't compile
	//let best_idx = seating_permutations.max_by_key(|ref item| score_arrangement(&happiness, &item));

	println!("Max score {}", max_score);
}
