
// There are infinitely many Elves, numbered starting with 1. Each Elf 
// delivers presents equal to ten times his or her number at each house.

fn calculate_infinate(target_num: usize) {

	// The last house is gaurenteed to get house_num*10 presents,
	// so use that as an upper bound on the number of houses to search.
	let upper_bound = target_num / 10 + 1;
	
	let mut houses = vec![0; upper_bound];
	
	// One elf starts per house number, starting at 1.
	let mut lowest_house_index = upper_bound;
	for elf_num in 1..houses.len() {
		
		// Walk the elf along the houses
		let mut house_num = elf_num;
		while house_num < houses.len() {
			
			houses[house_num] += elf_num * 10;
			
			// Save the house index if it has the required number of presents.
			// Future elves may visit lower houses, so we can't just exit here.
			if houses[house_num] >= target_num {
				lowest_house_index = std::cmp::min(lowest_house_index, house_num);
			}
			
			house_num += elf_num;
		}
	}
	
	println!("Part 1: House {} has {} presents", lowest_house_index, houses[lowest_house_index]);
	assert_eq!(lowest_house_index, 776160)
}


fn calculate_limit_50(target_num: usize) {

	// The last house is gaurenteed to get house_num*11 presents,
	// so use that as an upper bound on the number of houses to search.
	let upper_bound = f32::ceil(target_num as f32 / 11.0) as usize;
	
	let mut houses = vec![0; upper_bound];
	
	// One elf starts per house number, starting at 1.
	let mut lowest_house_index = upper_bound;
	for elf_num in 1..houses.len() {
		
		// Walk the elf along the houses
		let mut house_num = elf_num;
		let mut houses_visited = 0;
		while house_num < houses.len() && houses_visited < 50 {
			
			houses[house_num] += elf_num * 11;
			
			// Save the house index if it has the required number of presents.
			// Future elves may visit lower houses, so we can't just exit here.
			if houses[house_num] >= target_num {
				lowest_house_index = std::cmp::min(lowest_house_index, house_num);
			}
			
			house_num += elf_num;
			houses_visited += 1;
		}
	}
	
	println!("Part 2: House {} has {} presents", lowest_house_index, houses[lowest_house_index]);
	assert_eq!(lowest_house_index, 786240)
}

fn main() {

	let target_num = 33100000;
	calculate_infinate(target_num);
	calculate_limit_50(target_num);
}
