

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use std::collections::VecDeque;

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

// Try to parse the input as a constant. On falure, use as a wire ID.
// The explicit <'a> lifetime specifier denotes that the HashMap &str and input &str have equivelent lifetimes
fn get_input_value<'a>(wires: &mut HashMap<&'a str, u16>, input_str: &'a str) -> Option<u16> {
    let opt_val_in = input_str.parse::<u16>();
    match opt_val_in {
        Ok(input_value) => Some(input_value), // Parsed input as a constant integer
        Err(_) => {
            // Couldn't parse, so it must be a wire ID
            if input_str == "b" {
                Some(16076)
            } else {
                match wires.entry(input_str) {
                    Occupied(entry) => Some(*entry.get()),
                    Vacant(_) => None,
                }
            }
        }
    }
}

// Process gates of size 3. Returns true on success
fn process_gate_3<'a>(tokens: &Vec<&'a str>,
                      wires: &mut HashMap<&'a str, u16>,
                      verbose: bool)
                      -> bool {
    // WIRE type: 123 -> x
    let in_arg = tokens[0];
    let wire_id_out = tokens[2];

    let opt_value_out = get_input_value(wires, in_arg);

    if let Some(value_in_out) = opt_value_out {
        wires.insert(wire_id_out, value_in_out);
        if verbose {
            println!("{} ({}) => {} ({})",
                     in_arg,
                     value_in_out,
                     wire_id_out,
                     value_in_out);
        }
        true
    } else {
        false
    }
}

fn process_gate_4<'a>(tokens: &Vec<&'a str>,
                      wires: &mut HashMap<&'a str, u16>,
                      verbose: bool)
                      -> bool {
    // 1-arg NOT gate: NOT di -> dj
    let wire_id_in = tokens[1];
    let wire_id_out = tokens[3];

    // Try to find the input wire value
    let wire_opt_in = get_input_value(wires, wire_id_in);

    // If the wire input exists, invert and write to output.
    if let Some(wire_val_in) = wire_opt_in {
        let value_out = !wire_val_in;
        wires.insert(wire_id_out, value_out);
        if verbose {
            println!("NOT {} ({}) \t=> {} ({})",
                     wire_id_in,
                     wire_val_in,
                     wire_id_out,
                     value_out);
        }
        true
    } else {
        false
    }
}

fn process_gate_5<'a>(tokens: &Vec<&'a str>,
                      wires: &mut HashMap<&'a str, u16>,
                      verbose: bool)
                      -> bool {
    // 2-arg gate: et AND fe -> fg
    let gate_type = tokens[1];
    let wire_in_1 = tokens[0];
    let wire_in_2 = tokens[2];
    let wire_id_out = tokens[4];

    let wire_opt_1 = get_input_value(wires, wire_in_1);
    let wire_opt_2 = get_input_value(wires, wire_in_2);

    // If both inputs are valid, calculate the gate output
    if let (Some(input_1), Some(input_2)) = (wire_opt_1, wire_opt_2) {
        let value_out = match gate_type {
            "AND" => input_1 & input_2,
            "OR" => input_1 | input_2,
            "LSHIFT" => input_1 << input_2,
            "RSHIFT" => input_1 >> input_2,
            _ => {
                println!("Unknown pattern");
                0
            }
        };
        wires.insert(wire_id_out, value_out);
        if verbose {
            println!("{} ({}) {} {} ({}) \t=> {} ({})",
                     wire_in_1,
                     input_1,
                     gate_type,
                     wire_in_2,
                     input_2,
                     wire_id_out,
                     value_out);
        }
        true
    } else {
        false
    }
}

fn process_gate_queue<'a>(gates: &'a Vec<String>,
                          wires: &mut HashMap<&'a str, u16>,
                          verbose: bool) {

    // Create a local deque of references as a queue of gates
    let mut remaining_gates: VecDeque<&str> = gates.into_iter().map(AsRef::as_ref).collect();

    while remaining_gates.len() > 0 {

        let opt_gate = remaining_gates.pop_front().to_owned();
        let mut retry_gate = false;

        match opt_gate {
            Some(gate) => {

                let tokens: Vec<&str> = gate.split_whitespace().collect();
                let gate_length = tokens.len();

                if gate_length == 3 {
                    retry_gate = !process_gate_3(&tokens, wires, verbose);
                } else if gate_length == 4 {
                    retry_gate = !process_gate_4(&tokens, wires, verbose);
                } else if gate_length == 5 {
                    retry_gate = !process_gate_5(&tokens, wires, verbose);
                } else {
                    println!("UNKNOWN GATE TYPE: {}", gate);
                }

                if retry_gate {
                    if tokens[0] == "b" {
                        println!("{}", gate);
                    }
                    remaining_gates.push_back(gate); // Return the unprocessed gate to the queue
                }
            }
            None => (),
        } // match
    } // loop
}

// Main
fn main() {

    let gates = get_input_lines("day7.txt");
    let mut wires: HashMap<&str, u16> = HashMap::new();

    process_gate_queue(&gates, &mut wires, false);

    // Print output for part 1
    let wire_opt_a = get_input_value(&mut wires, "a");
    println!("Part 1: wire 'a' has value {:?}", wire_opt_a);

    println!("================================================");

    // Part 2:
    // Clear wires, set 'b' to 'a', and reprocess.
    let opt_old_a_value = get_input_value(&mut wires, "a");
    if let Some(old_a_value) = opt_old_a_value {
        wires.clear();
        wires.insert("b", old_a_value);

        println!("Part 2 in: {:?}", wires);

        process_gate_queue(&gates, &mut wires, false);

        let wire_opt_a_after = get_input_value(&mut wires, "a");
        println!("Part 2: wire 'a' has value {:?}", wire_opt_a_after)
    }
}
