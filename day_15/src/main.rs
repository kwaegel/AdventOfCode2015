
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

#[macro_use] extern crate itertools;

extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

extern crate regex;
use regex::Regex;

// --------------------------------------------------------

#[derive(Debug)]
struct Ingredient {
	name: String,
	capacity: i32,
	durability: i32,
	flavor: i32,
	texture: i32,
	calories: i32,
}

// --------------------------------------------------------

fn get_ingredients_from_file(filename: &str) -> Vec<Ingredient> {
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
	
	let re = Regex::new(r"(-?\d+),?").unwrap();
	
	let mut ingredients: Vec<Ingredient> = Vec::new();
	for line in lines {
		let string = line.unwrap();
		let tokens: Vec<&str> = string.split_whitespace().collect();
		let name = tokens[0];		
		
		let values: Vec<i32> = re.captures_iter(&string)
					   .map(|cap| cap.at(1).unwrap_or("").parse::<i32>().unwrap())
					   .collect();
					   
		let i = Ingredient { 
			name:name.to_string(),
			capacity:values[0], 
			durability:values[1], 
			flavor:values[2], 
			texture:values[3], 
			calories:values[4] };
			
		ingredients.push(i);		
	}
		
    ingredients
}

// --------------------------------------------------------

fn get_score(ingredients: &Vec<Ingredient>, quantities: &Vec<i32>) -> i32 {
	let mut sub_scores = vec![0i32; 4];
	let mut calories = 0;
	for (ing, amount) in ingredients.iter().zip(quantities.iter()) {
		sub_scores[0] += ing.capacity * amount;
		sub_scores[1] += ing.durability * amount;
		sub_scores[2] += ing.flavor * amount;
		sub_scores[3] += ing.texture * amount;
		calories += ing.calories * amount;
	}

	let raw_score = sub_scores.iter()
			        .map(|&x| cmp::max(0, x))
			        .fold(1, |prod, x| prod*x );
		
	let final_score = if calories > 500 { 0 } else { raw_score };
	//let final_score = raw_score;
	
	println!("Subscores are {:?} = {} with {} calories = {}", 
		sub_scores, raw_score, calories, final_score);
	
	final_score
}

// --------------------------------------------------------

fn get_random_quantities<R: Rng>(vec_size: usize, rng: &mut R) -> Vec<i32> {
	let mut nums = vec![0i32; vec_size];
	let mut sum = 0;
	for i in 0..vec_size-1 {
		let max = 100 - sum;
		let between = Range::new(0i32, max+1);
		nums[i] = between.ind_sample(rng);
		sum += nums[i];
	}
	nums[vec_size-1] = 100-sum;
	
	let vec_sum = nums.iter().fold(0, |sum, x| sum + x);
	assert_eq!(100, vec_sum);
	
	nums
}

// --------------------------------------------------------

fn main() {
	let mut rng = rand::thread_rng();

    let ingredients = get_ingredients_from_file("day15.txt");
	
	// Pick random quantities until score > 0
	let mut quantities = Vec::new();	
	let mut best_score = 0;	
	while best_score == 0 {
		// Generate a new initial solution until score > 0
		quantities = get_random_quantities(ingredients.len(), &mut rng);
		best_score = get_score(&ingredients, &quantities);
	}
	
	// Solve using hill climbing
	let mut keep_checking = true;
	let mut best_score = 0;
	let quant_count = quantities.len();
	while keep_checking {
		keep_checking = false;
		
		// Iterate over the inner product of possible changes
		for (inc_idx, dec_idx) in iproduct!(0..quant_count, 0..quant_count) {
			if inc_idx != dec_idx {
			
				// Walk in one direction and re-score the objective function
				quantities[inc_idx] += 1;
				quantities[dec_idx] -= 1;
				let score = get_score(&ingredients, &quantities);
				//println!("Quantities: {:?} = {}", &quantities, score);
				
				// If the score is better, save it and restart the search next iteration.
				if score > best_score {
					best_score = score;
					keep_checking = true;
					continue;
				} else {
					// Score went down. Restore previous values
					quantities[inc_idx] -= 1;
					quantities[dec_idx] += 1;
				}
			}
		}
	}
	
	let score = get_score(&ingredients, &quantities);
	println!("Part 2: score for {:?} is {}", &quantities, score);
}
