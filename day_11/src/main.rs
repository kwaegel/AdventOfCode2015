
use std::char;

// --------------------------------------------------------

// Returns true on overflow
fn increment_char(letter: &mut char) -> bool {
	if *letter == 'z' {
		*letter = 'a';
		return true;
	} else {
		*letter = char::from_u32(*letter as u32 + 1).unwrap();
		false
	}
}

// --------------------------------------------------------

// Mutate password in-plcae
fn increment_password(password: &mut Vec<char>) {
	assert_eq!(password.len(), 8);

	let mut char_idx = 7; // Last index
	let mut needs_increment = true;
	while needs_increment {
		needs_increment = increment_char(&mut password[char_idx]);
		if needs_increment {
			needs_increment = true;
			char_idx -= 1;
		}
	}
}

// --------------------------------------------------------

fn is_valid_password(password: &Vec<char>) -> bool {

	let mut has_sequence = false;
	let mut num_pairs = 0;

	let mut back_two = 999u32;
	let mut back_one = 999u32;
	for c in password {
		//print!("{} ", c);
		if *c == 'i' || *c == 'o' || *c == 'l' {
			return false;
		}
		
		
		if *c as u32 == back_one 
		   && *c as u32 != back_two {
			num_pairs += 1;
		}
		
		if *c as u32 == back_one + 1 
		   && back_one == back_two + 1 {
			has_sequence = true;
		}
		
		back_two = back_one;
		back_one = *c as u32;
	}
	
	has_sequence && num_pairs >= 2
}

// --------------------------------------------------------

fn print_password(password: &Vec<char>) {
	for c in password {
		print!("{}", *c);
	}
	println!("");
}

// Increment password until it passes muster
fn main() {
	
	let old_password = "cqjxjnds";
	let mut password_chars: Vec<char> = old_password.chars().collect();

	print_password(&password_chars);
	
	while !is_valid_password(&password_chars) {
		increment_password(&mut password_chars);
		//print_password(&password_chars);
	}
	
	let mut new_password: String = password_chars.clone().into_iter().collect();
	println!("Part 1: new password is {}", new_password);
	
	// Part 2: run it again...
	increment_password(&mut password_chars);
	while !is_valid_password(&password_chars) {
		increment_password(&mut password_chars);
	}
	
	new_password = password_chars.clone().into_iter().collect();
	println!("Part 2: new password is {}", new_password);
}
