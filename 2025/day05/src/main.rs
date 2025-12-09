use std::fs::File;
use std::io::{BufRead, BufReader};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader 	= BufReader::new(file);
	
	let mut range_mode: bool				= true;
	let mut boundary_bottom:	Vec<u64>	= Vec::new();
	let mut boundary_top: 		Vec<u64>	= Vec::new();
	
	let mut boundary_tuples:    Vec<(u64, u64)>	= Vec::new();
	let mut walk_top_fresh:		u64				= 0;
	
	let mut count_fresh: u64				= 0;
	let mut fresh_ids_count: u64			= 0;
	
	for line_result in reader.lines() {
		let line	= line_result.unwrap();
		
		if line.len() == 0 {
			range_mode	= false;
			continue;
		}
		
		if range_mode {
			match line.split_once('-') {
				Some((low, high)) => {
					
					let bottom: u64	= low.parse().expect("Cannot parse lower bound");
					let top: u64	=high.parse().expect("Cannot parse upper bound");
					boundary_bottom.push(bottom);
					boundary_top.push(top);
					boundary_tuples.push((bottom, top));
				},
				None => {
					panic!("Invalid range format");
				}
				
			}
		} else {
			let inv_id: u64 = line.parse().expect("Cannot parse inventory ID");
			if is_in_range(inv_id, &boundary_bottom, &boundary_top) {
				count_fresh +=1;
			}
		}
	}
	
	// RESULT part one
	println!("Fresh: {}", count_fresh);
	
	//Part two:
	boundary_tuples.sort_by_key(|&(key, _)| key);
	
	for idx in 0..boundary_tuples.len() {
		if idx == 0 {
			fresh_ids_count += boundary_tuples[idx].1 - boundary_tuples[idx].0 + 1;
			walk_top_fresh	 = boundary_tuples[idx].1;
		} else {
			if boundary_tuples[idx].1 <= walk_top_fresh {
				continue;
			}
			
			if boundary_tuples[idx].0 <= walk_top_fresh {
				fresh_ids_count	+= boundary_tuples[idx].1 - walk_top_fresh;
			} else {
				fresh_ids_count	+= boundary_tuples[idx].1 - boundary_tuples[idx].0 +1;
			}
			
			walk_top_fresh = boundary_tuples[idx].1;
		}
	}
	
	// RESULT part two
	println!("All fresh: {}", fresh_ids_count);
}

fn is_in_range(inventory_id: u64, bottoms: &Vec<u64>, tops: &Vec<u64> ) -> bool {
	for i in 0..bottoms.len() {
		if inventory_id >= bottoms[i] && inventory_id <= tops[i] {
			return true;
		}
 	}
  false
}