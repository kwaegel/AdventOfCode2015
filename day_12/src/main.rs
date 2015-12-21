
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json::*;

fn read_file_to_string(filename: &str) -> String {
	let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
	
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
	data
}

// --------------------------------------------------------

fn parse_json_array(json: &Vec<Json>, tree_depth: usize) -> i64 {
	
	print!("{: >1$}", " ", tree_depth);
	println!("{:?}", json);
	0
}

// --------------------------------------------------------

fn parse_json_tree(json: &Json, tree_depth: usize) -> i64 {
	
	// Json types: I64, U64, F64, Boolean, String, Array, Object, Null
	match *json {
		Json::Null 			=> { println!("(null)"); 0},
		Json::I64(v)     	=> { println!("{}", v);  v},
		Json::U64(v)     	=> { println!("{}", v);  v as i64},
		Json::F64(v)     	=> { println!("{}", v);  v as i64},
		Json::Boolean(b) 	=> { println!("{}", b);  0},
		Json::String(ref s) => { println!("{}", s);  0},
		Json::Array(ref arr) => {
			println!("(array)",);
			let mut sum = 0;
			for arr_obj in arr {
				print!("{: >1$}", " ", tree_depth);
				sum += parse_json_tree(arr_obj, tree_depth+1);
			}
			sum
		},
		Json::Object(ref obj) => {
			println!("(object)");
			let mut sum = 0;
			for (key, value) in obj.iter() {
				print!("{: >1$}", " ", tree_depth);
				print!("{}: ", key);
				sum += parse_json_tree(value, tree_depth+1);
				
				// Skip counting this object if a key contains "red"
				match *value {
					Json::String(ref s) => if s == "red" { return 0},
					_ => {},
				};
			}
			sum
		},
	}
}

// --------------------------------------------------------

fn main() {
	let strings = read_file_to_string("day12.txt");
	let json = Json::from_str(&strings).unwrap();
	let sum = parse_json_tree(&json, 0);
	
	println!("Tree sum is {}", sum);
}
