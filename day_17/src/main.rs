
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;


// Number of permutations summing to value N
fn find_permutation_sums(values: &[i32], target: i32) -> i32 {

	let indent = 21-values.len();
	//print!("{spacer:>0width$}", spacer=" ", width=indent);
	//println!("{:?} : {}", values, target);

	let mut permutations = 0;
	for i in 0..values.len() {
		let remaining_target = target - values[i];
		if remaining_target < 0 {
			// Current value is too big
			continue;
		} else if remaining_target == 0 {
			// Number is just the right size. Don't check extra values.
			permutations +=1;
			continue;
		} else if i < values.len() - 1 {
			// Check this value plus the sum from remaining values
			let subslice = &values[i+1..values.len()];
			permutations += find_permutation_sums(subslice, remaining_target);
		}		
	}
	permutations
}


fn main() {
	let path = Path::new("day17.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
	
    let reader = BufReader::new(file);
	let mut values: Vec<i32> = reader.lines().map(|so| so.unwrap())
							  .map(|s| s.parse::<i32>().unwrap())
							  .collect();
	
	values.sort_by(|a, b| b.cmp(a));	// Reverse sort
    println!("{:?}", values);	
	let num_permutations_150 = find_permutation_sums(&values[..], 150);
	println!("Number of values summing to 150 is {}", num_permutations_150);
	
	let test_values = vec![20, 15, 10, 5, 5];
	//println!("{:?}", test_values);	
	let num_permutations_25 = find_permutation_sums(&test_values[..], 25);
	//println!("Number of values summing to 25 is {}", num_permutations_25);
	assert_eq!(num_permutations_25, 4);
	
}
