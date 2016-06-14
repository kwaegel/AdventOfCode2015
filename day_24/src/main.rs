use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

fn make_bitset32(indices: &[usize]) -> u32 {
    indices.iter().fold(0, |bits, &i| bits | (1 << i as u32))
}

#[derive(Debug,Copy,Clone)]
struct Group {
    count: u32,
    weight: u64,
    qe: u64, // quantum entanglement
    bitset: u32,
}
impl Group {
    fn new(weights: &[u32], indices: &[usize]) -> Group {
        let count = indices.len() as u32;
        let weight = indices.iter().fold(0u64, |sum, &i| sum + weights[i] as u64);
        let qe = indices.iter().fold(1u64, |prod, &i| prod * weights[i] as u64);
        let bitset = make_bitset32(indices);
        
        Group{count: count, weight: weight, qe: qe, bitset: bitset}
    }
}

fn collect_indices(vals: &[bool]) -> Vec<usize> {
    vals.iter().enumerate().filter(|&(_, val)| *val == true).map(|(i, _)| i).collect()
}

// For each index in the array:
//  Add the corrosponding value to the current sum
//  If size equal to target size:
//      add to the list.
//  else
//      Recursively iterate over all indices larger than the current one
//  Unset the current index to remove it from the active set.
fn find_subsets(weights: &[u32],
                start: usize,
                target_size: u32,
                starting_size: u32,
                used_indices: &mut Vec<bool>,
                results: &mut Vec<Vec<usize>>) {

    for idx in start..weights.len() {
        let used = used_indices[idx];
        let weight = weights[idx];

        if !used && starting_size + weight <= target_size {
            used_indices[idx] = true;
            let current_size = starting_size + weight;
            if current_size == target_size {
                results.push(collect_indices(&used_indices));
            } else {
                find_subsets(&weights,
                             idx,
                             target_size,
                             current_size,
                             used_indices,
                             results);
            }

            used_indices[idx] = false;
        }
    }
}

fn main() {

    // Read weights to vector
    let path = Path::new("input.txt");
    let file = match File::open(&path) {
        Err(why) => {
            panic!("couldn't open {}: {}",
                   path.display(),
                   Error::description(&why))
        }
        Ok(file) => file,
    };

    let lines = BufReader::new(file).lines();
    // ok() converts Result into Option
    let weights: Vec<u32> = lines.into_iter()
                                 .map(|l| l.ok().and_then(|s| s.parse().ok()).unwrap_or(0))
                                 .collect();

    let total_weight = weights.iter().fold(0, |sum, &n| sum + n);
    let bin_size = total_weight / 3;

    println!("Trying to fit {} items of total weight {} into three bins of size {} each.",
             weights.len(),
             total_weight,
             bin_size);

    let mut used = vec![false; weights.len()];
    let mut results = Vec::new();
    find_subsets(&weights, 0, bin_size, 0, &mut used, &mut results);

    println!("Found {} sets of size {}", results.len(), bin_size);
    
    let mut groups: Vec<Group> = results.iter().map(|ref indices| Group::new(&weights, &indices)).collect();
    
    // Sort by package count
    groups.sort_by_key(|a| a.count);
    
    // Now that the groups are sorted by package order, find the first one that
    // has at least one other non-overlapping group (bits & bits) == 0
    // If two groups can exist at the same time, the third exists by default
    let mut lowest_count = u32::max_value();
    let mut lowest_qe = u64::max_value();
    let mut lowest_weight = u64::max_value();
    let mut best_group = Group{count: 0, weight:0, qe: 0, bitset: 0};
    for group_1 in &groups {
        if group_1.count > lowest_count {
            break;
        }
        
        for group_2 in &groups {
            let overlap = group_1.bitset & group_2.bitset;
            
            // If compatible and the weight has not increased
            if overlap == 0 && group_1.weight <= lowest_weight && group_1.qe < lowest_qe {
                lowest_count = cmp::min(lowest_count, group_1.count);
                lowest_weight = cmp::min(lowest_weight, group_1.weight);
                lowest_qe = cmp::min(lowest_qe, group_1.qe);
                best_group = group_1.clone();
            }
        }
    }
    
    println!("Found best group_1: {:?}", best_group);
    
    assert_eq!(best_group.qe, 11846773891);
}
