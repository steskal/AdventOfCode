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
	
	let mut count_fresh: u64				= 0;
	
	for line_result in reader.lines() {
		let line	= line_result.unwrap();
		
		if line.len() == 0 {
			range_mode	= false;
			continue;
		}
		
		if range_mode {
			match line.split_once('-') {
				Some((low, high)) => {
					
					boundary_bottom.push(low.parse().expect("Cannot parse lower bound"));
					boundary_top.push(high.parse().expect("Cannot parse upper bound"));
					
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
	
	// RESULT
	println!("Fresh: {}", count_fresh);
}

fn is_in_range(inventory_id: u64, bottoms: &Vec<u64>, tops: &Vec<u64> ) -> bool {
	for i in 0..bottoms.len() {
		if inventory_id >= bottoms[i] && inventory_id <= tops[i] {
			return true;
		}
 	}
  false
}