use std::fs::File;
use std::io::{BufRead, BufReader};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader	= BufReader::new(file);
	
	let mut add_up	= 0;
	
	for line_result in reader.lines() {
		let line 				= line_result.unwrap();	// will panic, if the line cannot be read - that's OK
		let ranges: Vec<&str> 	= line.split(',').collect();
		
		/* Iterate through result vector */
		for range in ranges {
			match range.split_once('-') {
				Some((range_start, range_end)) => {
					add_up += add_invalid_ids_in_range(range_start, range_end);
				}
				None => {
					let range_start: &str	= range;
					let range_end: &str		= range;
					
					add_up += add_invalid_ids_in_range(range_start, range_end);
				}
			}
		}
	}
	
	println!("Total invalid IDs: {}", add_up);
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

/* Check if provided id consist exactly of 2 repeated sequences of digits. eg: 99, 123123 */
fn is_repeated2(id:u64) -> bool {
	let id_str	= id.to_string();
	let (first, last) =id_str.split_at(id_str.len() / 2);
	
	if first == last {
		return true;
	}
	return false;
}
