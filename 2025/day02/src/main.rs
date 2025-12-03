use std::fs::File;
use std::io::{BufRead, BufReader};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader	= BufReader::new(file);
	
	let mut add_up			= 0;
	let mut add_up_part_2	= 0;
	
	for line_result in reader.lines() {
		let line 				= line_result.unwrap();	// will panic, if the line cannot be read - that's OK
		let ranges: Vec<&str> 	= line.split(',').collect();
		
		/* Iterate through result vector */
		for range in ranges {
			match range.split_once('-') {
				Some((range_start, range_end)) => {
					add_up			+= add_invalid_ids_in_range(range_start, range_end);
					add_up_part_2	+= add_invalid_ids_in_range_2(range_start, range_end);
				}
				None => {
					let range_start: &str	= range;
					let range_end: &str		= range;
					
					add_up			+= add_invalid_ids_in_range(range_start, range_end);
					add_up_part_2	+= add_invalid_ids_in_range_2(range_start, range_end);
				}
			}
		}
	}
	
	println!("Total invalid IDs for part one: {}", add_up);
	println!("Total invalid IDs for part two: {}", add_up_part_2);
}

/* Find invalid IDs in a range and sum them together.
 * Invalid id is a string consisting of same two sequences of digits, eg: 99, 123123, 222222 etc.
 */
fn add_invalid_ids_in_range(range_start: &str, range_end: &str)  -> u64 {
	let mut added = 0;
	let start:u64	= range_start.parse().expect(&format!("Couldn't parse range start from: [{}, {}]", range_start, range_end));
	let end:u64		= range_end.parse().expect(&format!("Couldn't parse range start from: [{}, {}]", range_start, range_end));
	
	for current_id in start..=end {
		if is_repeated2(current_id) {
			added += current_id;
		}
	}
	return added;
}

/* This function could replace the previous function but I'm keeping it here 
 * so I can keep the code for the first part of the challenge in.
 */
 fn add_invalid_ids_in_range_2(range_start: &str, range_end: &str) -> u128 {
 	let mut added 	=0;
  	let start: u64 	= range_start.parse().expect(&format!("Couldn't parse range start from: [{}, {}]", range_start, range_end));
   	let end:u64		= range_end.parse().expect(&format!("Couldn't parse range start from: [{}, {}]", range_start, range_end));
    
    for current_id in start..=end {
    	if is_repeated(current_id) {
     		added	+= current_id as u128;
      	}
    }
  
  	return added;
 }

/* Check if provided id consist exactly of 2 repeated sequences of digits. eg: 99, 123123 */
fn is_repeated2(id:u64) -> bool {
	let id_str	= id.to_string();
	let (first, last) =id_str.split_at(id_str.len() / 2);
	
	if first == last {
		return true;
	}
	return false;
}

fn is_repeated(id:u64) -> bool {
	let id_str 	= id.to_string();
	let len		= id_str.len();
	
	/* Cycle through different pattenr lengths from 1/2 or the len down to 1.
	 * Cycling down seems to be more efficient, than xyxling up, but I don't have a proof.
	 */
	for pattern_len in (1..=len/2).rev() {
		if len % pattern_len == 0 {	// Only if repeated pattern can fill the whole string
			let pattern 		= &id_str[0..pattern_len];
			let mut is_repeated = true;
			
			/* Check if pattern repeats */
			for idx in (pattern_len..len).step_by(pattern_len) {
				if &id_str[idx..idx+pattern_len] != pattern {
					is_repeated = false;
					break;
				}
			}
			
			/* if the is repeated remained true - pattern is repeated throughout the whole string */
			if is_repeated {
				return true;
			}
		}
	}
	return false;
}
