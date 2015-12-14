
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

extern crate nalgebra as na;
use na::{DMat};

fn get_input_lines(filename : &str) -> Vec<String> {
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

#[derive(Debug)]
struct Range<T> {
	start: T,
	end: T,
}

#[derive(Debug)]
enum NewState {
	Toggle,
	On,
	Off,
}

#[derive(Debug)]
struct Command {
	new_state: NewState,
	x_range: Range<usize>,
	y_range: Range<usize>,
}

fn parse_range(range_str: &str) -> (usize, usize) {
	let tokens: Vec<&str> = range_str.split(",").collect();
	let x = tokens[0].parse::<usize>().unwrap();
	let y = tokens[1].parse::<usize>().unwrap();
	(x, y)
}

// Formats:
// 			turn off 12,823 through 102,934
// 			toggle 756,965 through 812,992
fn parse_command(command_string : &String) -> Command {

	let tokens: Vec<&str> = command_string.split_whitespace().collect();
	
	let command = match tokens[1] {
		"on" => NewState::On,
		"off" => NewState::Off,
		_ => NewState::Toggle,
	};
	
	// Pick indices of the last, and third from last tokens
	let (x1, y1) = parse_range(tokens[tokens.len()-3]);
	let (x2, y2) = parse_range(tokens[tokens.len()-1]);
	let x_range = Range {start: x1, end: x2+1};
	let y_range = Range {start: y1, end: y2+1};
	
	Command {new_state:command, x_range:x_range, y_range:y_range}
}

fn toggle_lights(grid: &mut [[bool; 1000]; 1000], cmd: &Command) {

	// TODO: iterator is ugly. Find a better way.
	for y_idx in cmd.y_range.start..cmd.y_range.end {
		for x_idx in cmd.x_range.start..cmd.x_range.end {
			grid[y_idx][x_idx] = 
				match cmd.new_state {
					NewState::On => true,
					NewState::Off => false,
					NewState::Toggle => !grid[y_idx][x_idx],
				}
		}
	}
}

fn change_light_brightness(grid: &mut DMat<i32>, cmd: &Command) {
	
	for y_idx in cmd.y_range.start..cmd.y_range.end {
		for x_idx in cmd.x_range.start..cmd.x_range.end {
			grid[(y_idx,x_idx)] = 
				match cmd.new_state {
					NewState::On => grid[(y_idx,x_idx)] + 1,
					NewState::Off => cmp::max(0, grid[(y_idx,x_idx)] - 1),
					NewState::Toggle => grid[(y_idx,x_idx)] + 2,
				};
		}
	}
}


fn main() {
	let lines = get_input_lines("day6.txt");
	let command_list: Vec<Command> = lines.iter().map(|x| parse_command(x)).collect();
	
	let mut bool_grid = [[false; 1000]; 1000];
	for cmd in &command_list {
		toggle_lights(&mut bool_grid, &cmd);
	}
	
	// Count lights
	let lights_on = bool_grid.iter()
		.fold(0, |sum, &x| sum + x.iter().filter(|&y| *y).count());
		
	println!("There are {} lights on", lights_on);
	
	
	// Part 2: change brightness
	// Need to use a heap-allocated DMat, since stack allocation will overflow.
	let mut light_grid: DMat<i32> = DMat::new_zeros(1000, 1000);
	for cmd in &command_list {
		change_light_brightness(&mut light_grid, &cmd);
	}
	
	// Count lights
	let total_brightness = light_grid.as_vec().iter()
		.fold(0, |sum, &x| sum + x);
		
	println!("The total brightness is {} ", total_brightness);
}
