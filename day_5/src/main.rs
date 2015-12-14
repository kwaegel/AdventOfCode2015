
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like  xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

fn is_nice_string(string: &String) -> bool {
	let has_bad_str = !string.contains("ab") &&
					  !string.contains("cd") &&
					  !string.contains("pq") &&
					  !string.contains("xy");
					  
	let mut vowels = 0;
	let mut last_char = '.';
	let mut has_double = false;
	
	for c in string.chars() {
		if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
			vowels += 1;
		}
		
		if c == last_char {
			has_double = true;
		}
		
		last_char = c;
	}
					  
	has_bad_str && vowels >= 3 && has_double
}


// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
fn is_nicer_string(string: &String) -> bool {

	let mut has_min_two_pairs = false;
	let mut has_sep_dupe = false;
	let mut one_back = '.';
	let mut two_back = '.';
	for c in string.chars() {
		if c == two_back {
			has_sep_dupe = true;
		}
		two_back = one_back;
		one_back = c;
		
		// Check for pattern in the remaining string
		let pattern = two_back.to_string() + &one_back.to_string();
		let match_count = string.matches(&pattern).count();
		
		if match_count > 1 {
			has_min_two_pairs = true;
		}
	}
	
	if has_sep_dupe && has_min_two_pairs {
		println!("{}", string);
	}
	
	has_sep_dupe && has_min_two_pairs
}


fn main() {
    // Create a path to the desired file
    let path = Path::new("day5.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
	
	let reader = BufReader::new(file);
	let lines = reader.lines();
	
	// let nice_results: Vec<String> = lines.map(|x| x.unwrap())
						// .filter(|x| is_nice_string(x))
						// .collect::<Vec<String>>();
						
	// let mut nice_count = 0;
	// for string in nice_results{
		// //println!("{}", string);
		// nice_count += 1;
	// }
	// println!("nice string results = {}", nice_count);
	
	let test = "abcXXXabcYYYabc".to_string();
	let v: Vec<&str> = test.matches("abc").collect();
	assert_eq!(v, ["abc", "abc", "abc"]);
	
	let nicer_results: Vec<String> = lines.map(|x| x.unwrap())
						.filter(|x| is_nicer_string(x))
						.collect::<Vec<String>>();
						
	let mut nice_count = 0;
	for string in nicer_results{
		println!("{}", string);
		nice_count += 1;
	}
	println!("nicer string results = {}", nice_count);



}
