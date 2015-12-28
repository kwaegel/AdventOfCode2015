
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

#[macro_use] extern crate itertools;

extern crate nalgebra as na;
use na::{DMat};

fn read_grid(filename: &str) -> DMat<i32> {
	
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
	
	// Note: uses [row, col] indexing
	let mut row_major_values = Vec::new();
	for line in lines {
		let string = line.unwrap();
		for c in string.chars() {
			let value = match c {
				'#' => 1,
				_ => 0,
			};
			row_major_values.push(value);
		}
	}
	
	let dims = (row_major_values.len() as f32).sqrt() as usize;
	DMat::from_row_vec(dims, dims, &row_major_values)
}

// --------------------------------------------------------

fn iterate(grid: &DMat<i32>) -> DMat<i32> {
	let mut grid_out: DMat<i32> = DMat::new_zeros(grid.nrows(), grid.ncols());
	
	for col in 0..grid_out.ncols() {
		for row in 0..grid_out.nrows() {
		
			// Half-open ranges, so use max+2
			let min_col = cmp::max(0, col as i32 - 1) as usize;
			let max_col = cmp::min(grid.ncols(), col+2);
			let min_row = cmp::max(0, row as i32 - 1) as usize;
			let max_row = cmp::min(grid.nrows(), row+2);
			
			let mut neighbor_sum = 0;
			for (icol, irow) in iproduct!(min_col..max_col, min_row..max_row) {
				neighbor_sum += grid[(icol, irow)];
			}
			
			// Subtract the cell itself
			neighbor_sum -= grid[(col, row)];
		
			if neighbor_sum == 3 {
				grid_out[(col, row)] = 1;
			} else if neighbor_sum == 2 && grid[(col, row)] == 1 {
				grid_out[(col, row)] = 1;
			} else {
				grid_out[(col, row)] = 0;
			}
		}
	}
	
	grid_out
}

// --------------------------------------------------------

fn sum_grid(grid: &DMat<i32>) -> i32 {
	
	let mut sum = 0;
	for col in 0..grid.ncols() {
		for row in 0..grid.nrows() {
			sum += grid[(col, row)];
		}
	}
	sum
}

// --------------------------------------------------------


fn main() {
    println!("Hello, world!");
	
	let mut test_grid = read_grid("day18_test.txt");
	println!("{:?}", test_grid);
	
	for step in 1..5 {
		println!("\nAfter {} steps", step);
		test_grid = iterate(&test_grid);
		println!("{:?}", test_grid);
	}
	
	let mut grid = read_grid("day18.txt");
	for _ in 1..101 {
		grid = iterate(&grid);
	}
	let lights_on = sum_grid(&grid);
	println!("Part 1: lights on {}", lights_on);
	assert_eq!(lights_on, 821);
}
