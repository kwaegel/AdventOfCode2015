 #[allow(unused_imports)]

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::char;

extern crate regex;
use regex::Captures;

extern crate rustc_serialize;
use rustc_serialize::hex::FromHex;

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

fn strip_escape_chars(text: &String) -> String {
	use regex::Regex;
	
	//println!("{}", text);
	
	// Identifies a backslash-hex value and converts the hex to a single char
	// i.e. \x27 => 39 => '\''
	let re_hex = Regex::new(r"\\[x]([0-9a-f]{2})").unwrap();
	let result = re_hex.replace_all(text, |caps: &Captures| {
		let chr_code: u32 = caps.at(1).unwrap_or("").from_hex().unwrap()[0] as u32;
		let chr = char::from_u32(chr_code).unwrap();
		//println!("\t{:?} -> {:?}", caps.at(1).unwrap_or(""), chr);
		chr.to_string()
	});
	
	// Replace escaped quotes and backslashes
	let result2 = result.replace(r#"\""#, "\"").replace(r"\\", r"\");
	
	// Strip the first and last quotes
	let re_quotes = Regex::new(r"(^.|.$)").unwrap();
	let result3 = re_quotes.replace_all(&result2, "");
	
	println!("{0: <45}  =>  {1: <45}", text, result3);
	result3
}

fn main() {
	let strings = get_input_lines("day8.txt");
	
	let mut total_raw = 0;
	let mut total_str = 0;
	
	for string in strings {
	
		let stripped_string = strip_escape_chars(&string);	
		
		let raw_length = string.chars().count();
		let stripped_length = stripped_string.chars().count();
		
		total_raw += raw_length;
		total_str += stripped_length;
		
		//println!("{:?} => {:?} \t::\t {:?} -> {:?}", string, stripped_string,
		// raw_length, stripped_length);
	}
	
	let delta = total_raw - total_str;
	println!("{} - {} = {}", total_raw, total_str, delta);
}
