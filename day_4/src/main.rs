extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn md5(input: &str) -> String {
	let mut digest = Md5::new();
	digest.input(&input.as_bytes());
	
	let result_str = digest.result_str();
	return result_str;
}

fn check_md5_value(buffer: [u8; 16]) -> bool {
	buffer[0] == 0 
	&& buffer[1] == 0 
	&& buffer[2] == 0
}

fn print_md5_buff(buffer: [u8; 16]) {

	for i in 0..16 {
		print!("{:02x} ", buffer[i]);
	}
	println!("");
}
 
fn main() {

	let test_val = "abcdef609043";
	let test_hash = "000001dbbfa3a5c83a2d506429c7b00e";
	assert!(md5(test_val) == test_hash);
	
	// Md5 hashes are 128 bits (16 bytes)
	let mut digest = Md5::new();
	let mut buffer: [u8; 16] = [0; 16];

	// Search for hash with "00000" prefix
	let input_prefix = "iwrupvqb".to_string();
	for num in 1..10000000 {
		let num_str = num.to_string();
		let value = String::new() + &input_prefix + &num_str;
		
		digest.input(&value.as_bytes());
		digest.result(&mut buffer);
		
		if check_md5_value(buffer) {
			print_md5_buff(buffer);
			println!("Value = {}", num_str);
			break;
		}
		
		digest.reset();		

	}
	
}