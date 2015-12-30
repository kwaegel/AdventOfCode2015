
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashSet;

struct Mapping {
	input: String,
	output: String,
}

// --------------------------------------------------------

fn read_input(filename: &str) -> (Vec<Mapping>, String) {
	
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
	
	let mut mapping_tuples = Vec::new();
	let mut input_str = String::new();
	
	for line in lines {
		let string = line.unwrap();
		let tokens: Vec<&str> = string.split_whitespace().collect();
		if tokens.len() > 1 {
			// form A => B
			let input = tokens[0].to_string();
			let output = tokens[2].to_string();
			//println!("Rule: {} -> {}", &input, &output);
			mapping_tuples.push( Mapping{input:input, output:output} );
		} else if tokens.len() == 1 {
			input_str = string.to_owned();
			//println!("Input string: {}", &input_str);
		}
	}
	
	(mapping_tuples, input_str)
}

// --------------------------------------------------------

fn main() {
	let (mapping_list, input) = read_input("day19.txt");
	
	let mut output_set = HashSet::new();
	
	let mut previous_c = '.';
	for (idx, c) in input.chars().enumerate() {
	
		let chr = c.to_string();
		let mut pair = previous_c.to_string();
		pair.push(c);
		//println!("Testing {} and {}", pair, chr);
		
		// Loop to check replacement rules
		for mapping in &mapping_list {
		
			// Note: string slicing is not safe for multi-byte characters	
			if chr == mapping.input || pair == mapping.input {
			
				let post_bytes = &input[idx+1..];				
				let replacement = &mapping.output;
			
				// 'idx' if replacing one character, idx-1 if replacing two.
				let prior_idx = idx +1 - mapping.input.len();
				let prior_bytes = &input[0..prior_idx];
			
				let result = format!("{}{}{}", &prior_bytes, &replacement, &post_bytes);
				output_set.insert(result);
			}
		}
		
		previous_c = c;
	}
	
	let num_unique = output_set.len();
	println!("Part 1: there are {} unique strings.", num_unique);
	assert_eq!(num_unique, 535);
	
	
}
