
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

// Try to parse the input as a constant. On falure, use as a wire ID.
fn get_input_value<'a>(wires: &mut HashMap<&'a str, u16>, input_str: &'a str) -> Option<u16> {
	let opt_val_in = input_str.parse::<u16>();
	match opt_val_in {
		Ok(input_value) => Some(input_value), // Parsed input as a constant integer
		Err(_) => {							  // Couldn't parse, so it must be a wire ID
			match wires.entry(input_str) {
				Occupied(entry) => Some(*entry.get()),
				Vacant(_) => None,
			}
		},
	}
}

// Check if the wire is unset
fn wire_is_unset<'a>(wires: &mut HashMap<&'a str, u16>, input_str: &'a str) -> bool {
	match wires.entry(input_str) {
		Occupied(_) => false,
		Vacant(_) => true,
	}
}

// AND, OR, LSHIFT, RSHIFT
fn calc_gate(gate_type: &str, input_1: u16, input_2: u16) -> u16 {
	match gate_type {
		"AND" => input_1 & input_2,
		"OR" => input_1 | input_2,
		"LSHIFT" => input_1 << input_2,
		"RSHIFT" => input_1 >> input_2,
		_ => { println!("Unknown pattern"); 0},
	}
}

// Main
fn main() {
	let gates = get_input_lines("day7.txt");

	let mut wires: HashMap<&str, u16> = HashMap::new();

	let mut repeated_gates = gates.iter().cycle();
	let mut num_processed_gates = 0;

	println!("Processing {} gates", gates.len());

	// Parse gates until none remain.
	while num_processed_gates < gates.len() {
		let l = repeated_gates.next().unwrap();
		let tokens: Vec<&str> = l.split_whitespace().collect();

		let gate_length = tokens.len();
		// match gate_length {
			// 3 => println!("WIRE: {}", l),
			// 4 => println!("1-arg GATE: {}", l),
			// 5 => println!("2-arg GATE: {}", l),
			// _ => println!("unknown: {}", l),
		// }

		if gate_length == 3 {
			// WIRE type: 123 -> x
			let in_arg = tokens[0];
			let wire_id_out = tokens[2];

			let opt_value_out = get_input_value(&mut wires, in_arg);

			if wire_is_unset(&mut wires, wire_id_out) {
				if let Some(value_out) = opt_value_out {
					wires.insert(wire_id_out, value_out);
					num_processed_gates += 1;
					println!("{}\tSet wire {} to {}", l, wire_id_out, value_out);
				}
			}
		} else if gate_length == 4 {
			// 1-arg NOT gate: NOT di -> dj
			let wire_id_in = tokens[1];
			let wire_id_out = tokens[3];

			// Try to find the input wire value
			let wire_opt_in = get_input_value(&mut wires, wire_id_in);

			// If the wire input exists, invert and write to output.
			if wire_is_unset(&mut wires, wire_id_out) {
				if let Some(wire_val_in) = wire_opt_in {
					let wire_val_out = !wire_val_in;
					wires.insert(wire_id_out, wire_val_out);
					num_processed_gates += 1;
					println!("{}\tSet wire {} to {}", l, wire_id_out, wire_val_out);
				}
			}
		} else if gate_length == 5 {
			// 2-arg gate: et AND fe -> fg
			let gate_type = tokens[1];
			let wire_in_1 = tokens[0];
			let wire_in_2 = tokens[2];
			let wire_id_out = tokens[4];

			let wire_opt_1 = get_input_value(&mut wires, wire_in_1);
			let wire_opt_2 = get_input_value(&mut wires, wire_in_2);

			// If both inputs are valid, calculate the gate output
			if wire_is_unset(&mut wires, wire_id_out) {
				if let (Some(val_1), Some(val_2)) = (wire_opt_1, wire_opt_2) {
					let value_out = calc_gate(gate_type, val_1, val_2);
					wires.insert(wire_id_out, value_out);
					num_processed_gates += 1;
					println!("{}\tSet wire {} to {}", l, wire_id_out, value_out);
				}
			}
		} else
		{
			println!("UNKNOWN GATE TYPE: {}", l);
		}
	}

	// // print wire values
	// for (wire_id, val) in &wires {
		// println!("{}: {}", wire_id, val);
	// }

	// Print output for part 1
	let wire_opt_a = get_input_value(&mut wires, "a");
	println!("Part 1: wire 'a' has value {:?}", wire_opt_a)
}
