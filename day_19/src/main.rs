
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashSet;

extern crate regex;
use regex::Regex;

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
	
	
	// Part 2: Reduce the string to else
	// Notes
	//  * The rules are in the form
	//       e => XX
	//		 X => XX
	//		 X => X Rn X Ar | X Rn X Y X Ar | X Rn X Y X Y Ar
	//  * Thinking of Rn and Ar as () and Y as , we get
	//       e => XX
	//		 X => XX
	//		 X => X(X) | X(X,X) || X(X,X,X)
	// To reduce a string of normal tokens to a single token takes tokens.len() - 1
	// To reduce a string of X(X) tokens takes tokens.len() - perens.len() - 1
	// To reduce a string of X(X,X,X) is the same as above, but an extra two for each the ,X
	
	let mut working = input.clone();
	working = working.replace("Rn", "(");
	working = working.replace("Ar", ")");
	working = working.replace("Y", ",");
	//println!("Working str: {}", &working);
	
	// Convert double-character tokens to 'X's for counting
	let re = Regex::new(r"[:upper:]{1}[:lower:]{1}").unwrap();
	working = re.replace_all(&working, "X");

	//println!("Working str: {}", &working);
	
	let token_count = working.len(); // FIXME: this needs to count tokens, not characters.
	let peren_count = working.matches("(").count() + working.matches(")").count();
	let sep_count = working.matches(",").count();
	
	let steps = token_count - peren_count - 2*sep_count - 1;
	println!("Part 2: Required steps is {}", steps);
	assert_eq!(steps, 212);
}
