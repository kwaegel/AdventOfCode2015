
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;


// Number of permutations summing to value N
// Values: container sizes
// target: remaining eggnog to be stored
// prior_used: containers used prior to this call
// hist: histogram of container counts
fn find_permutation_sums(values: &[i32], target: i32, prior_used: usize, hist: &mut Vec<i32>) {

	//let indent = 21-values.len();
	//print!("{spacer:>0width$}", spacer=" ", width=indent);
	//println!("{:?} : {}", values, target);

	for i in 0..values.len() {
		let remaining_target = target - values[i];
		if remaining_target < 0 {
			// Current value is too big
			continue;
		} else if remaining_target == 0 {
			// Number is just the right size. Don't check extra values.
			hist[prior_used + 1] += 1; // Record number of containers used
			continue;
		} else if i < values.len() - 1 {
			// Check this value plus the sum from remaining values
			let subslice = &values[i+1..values.len()];
			find_permutation_sums(subslice, remaining_target, prior_used + 1, hist);
		}		
	}
}


fn main() {

	{
		let test_values = vec![20, 15, 10, 5, 5];
		let mut hist = vec![0; test_values.len()];
		//println!("{:?}", test_values);
		find_permutation_sums(&test_values[..], 25, 0, &mut hist);
		let num_permutations_25 = hist.iter().fold(0, |sum, x| sum + x);
		//println!("{:?}", hist);
		//println!("Number of test values summing to 25 is {}", num_permutations_25);
		assert_eq!(num_permutations_25, 4);
	}
	
	

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
    println!("Input values: {:?}", values);	
	
	let mut histogram = vec![0; values.len()];
	find_permutation_sums(&values[..], 150, 0, &mut histogram);
	let num_permutations_150 = histogram.iter().fold(0, |sum, x| sum + x);
	assert_eq!(num_permutations_150, 4372);
	println!("Part 1: Number of permutations summing to 150 is {}", num_permutations_150);
	println!("Part 2: histogram {:?}", histogram);
	assert_eq!(histogram[4], 4);
	
}
