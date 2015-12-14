
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

struct Size {
	total_paper: i32,
	total_ribbon: i32,
}

fn get_sizes(line: String) -> Size {
	// Parse the file line as a vector of dimensions
	// map() applies the function parse() to each element X in the split iterator.
	let mut dimensions : Vec<i32> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
	let l = dimensions[0];
	let w = dimensions[1];
	let h = dimensions[2];
	
	// Calculate paper
	let sizes = vec![2*l*w, 2*w*h, 2*h*l];
	
	let mut min = std::cmp::min(sizes[0], sizes[1]);
	min = std::cmp::min(min, sizes[2]) / 2;
	
	let sizes_sum = sizes.iter().fold(0, |sum, x| sum + x);
	let total_paper = sizes_sum + min;
	
	println!("Paper: {}x{}x{} = {}+{}+{} + {} = {}", 
		l, w, h,
		sizes[0], sizes[1], sizes[2],
		min, total_paper);
	
	// Calculate ribbon
	dimensions.sort();
	let smallest_face_perimeter = dimensions[0]*2 + dimensions[1]*2;
	let volume = l*w*h;
	let total_ribbon = volume + smallest_face_perimeter;
	
	return Size {total_paper: total_paper, total_ribbon: total_ribbon};
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("day2.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
	
	let reader = BufReader::new(file);
	let lines = reader.lines(); 
	
	let mut total_paper = 0;
	let mut total_ribbon = 0;
	for l in lines {
		let line = l.unwrap();
		let package_sizes = get_sizes(line);
		total_paper += package_sizes.total_paper;
		total_ribbon += package_sizes.total_ribbon;
	}
		
	println!("Total paper is {}", total_paper);
	println!("Total ribbon is {}", total_ribbon);
}