
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::cmp;
use std::collections::HashMap;

extern crate nalgebra as na;
use na::{DMat};

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

// ----------------------------------------------------------------------------

fn get_city_index<'a>(city_indices: &mut HashMap<&'a str, usize>, name: &'a str) -> usize {
	let next_open_index = city_indices.len();
	*city_indices.entry(name).or_insert(next_open_index)
}

// ----------------------------------------------------------------------------

fn create_distance_mapping<'a>(strings: &'a Vec<String>) -> (HashMap<&'a str, usize>, DMat<u32>) {
	
	let mut city_indices: HashMap<&str, usize> = HashMap::new();
	let mut distances: DMat<u32> = DMat::new_zeros(8,8);
	
	for string in strings {
		let tokens: Vec<&str> = string.split_whitespace().collect();
		let start_name = tokens[0];
		let start_idx = get_city_index(&mut city_indices, start_name);		
		let end_name = tokens[2];
		let end_idx = get_city_index(&mut city_indices, end_name);		
		let distance = tokens[4].parse::<u32>().unwrap();
		
		distances[(start_idx, end_idx)] = distance;
		distances[(end_idx, start_idx)] = distance;
		//println!("{} ({}) => {} ({}) = {}", &start_name, start_idx, &end_name, end_idx, distance);		
	};
	
	(city_indices, distances)
}

// ----------------------------------------------------------------------------

fn find_min_path(distances: &DMat<u32>, city_indices: &Vec<usize>, previous_city: usize) -> u32 {

	// Starting case: for each starting city
	let is_starting_city = city_indices.len() == distances.nrows();

	// Base case: one index left to visit
	if city_indices.len() == 1 {
		let next_city = city_indices[0];
		return distances[(previous_city, next_city)];
	}
	
	// Recursive case: shortest path through the remaining cities
	let mut best_distance = u32::max_value();
	for i in 0..city_indices.len() {
		let mut remaining_indices = city_indices.clone();
		let next_city = remaining_indices.swap_remove(i);
		
		// Handle special case of being in the starting city (distance zero to previous).
		let this_dist = if is_starting_city {
			0u32
		} else {
			distances[(previous_city, next_city)]
		};
		
		let total_dist = this_dist + find_min_path(distances, &remaining_indices, next_city);
		best_distance = cmp::min(total_dist, best_distance);
	}
	
	best_distance
}

// ----------------------------------------------------------------------------

fn find_max_path(distances: &DMat<u32>, city_indices: &Vec<usize>, previous_city: usize) -> u32 {

	// Starting case: for each starting city
	let is_starting_city = city_indices.len() == distances.nrows();

	// Base case: one index left to visit
	if city_indices.len() == 1 {
		let next_city = city_indices[0];
		return distances[(previous_city, next_city)];
	}
	
	// Recursive case: shortest path through the remaining cities
	let mut best_distance = u32::min_value();
	for i in 0..city_indices.len() {
		let mut remaining_indices = city_indices.clone();
		let next_city = remaining_indices.swap_remove(i);
		
		// Handle special case of being in the starting city (distance zero to previous).
		let this_dist = if is_starting_city {
			0u32
		} else {
			distances[(previous_city, next_city)]
		};
		
		let total_dist = this_dist + find_max_path(distances, &remaining_indices, next_city);
		best_distance = cmp::max(total_dist, best_distance);
	}
	
	best_distance
}

// ----------------------------------------------------------------------------

fn main() {
	let strings = get_input_lines("day9.txt");
    let (city_indices_map, distances) = create_distance_mapping(&strings);
	
	println!("Checking {} cities", city_indices_map.len());
	
	// Find the shortest path from the last city through the remaining cities.
	let city_indices: Vec<usize> = city_indices_map.values().map(|&x| x).collect();
	let min_path_length = find_min_path(&distances, &city_indices, 0);
	println!("Part 1: min path length is {}", min_path_length);
	assert_eq!(min_path_length, 207); // 605 for test data
	
	// Find the longest path from the last city through the remaining cities.
	let max_path_length = find_max_path(&distances, &city_indices, 0);
	println!("Part 1: max path length is {}", max_path_length);
}
