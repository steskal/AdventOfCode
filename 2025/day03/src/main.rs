use std::fs::File;
use std::io::{BufRead, BufReader};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file 	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader 	= BufReader::new(file);
	
	let mut	total_output_part1:u64	= 0;
	
	for line_result in reader.lines() {
		let line 	= line_result.unwrap();	// will panic, if the line cannot be read
		
		//TEST
		println!("TEST line: {}", line);
		total_output_part1 += largest_joltage_2(&line) as u64;
		
	}
	println!("Part 1: {}", total_output_part1);
}

fn largest_joltage_2 (input: &str) -> u8 {
	let mut joltage 		= 0;
	let mut	digit_map: Vec<(usize, u8)> = input.chars()
											.enumerate()			 // enumerate return (index, character) for each character in the string
											.filter_map(|(pos, ch)| {  // take the (index, character and try to conver it to digit - filter outomatically filters out those, that cannot be converted (ch.to_digit(10) => None)
												ch.to_digit(10)		 // convert it to decimal digit (10 -> base -> decimal)
													.map(|digit| (pos as usize, digit as u8)) // Take the converted digit and map it together with positio to resultin tuple as correct target data types												
											}).collect();	//And collect them all to a vector
	
	digit_map.sort_by(|a, b| a.1.cmp(&b.1).reverse());	// Sort the vector by value (second field in the tuple)
	
	/* If the highest digit isnt in the last position, get reference for the position of the first element of the sorted vector - or fail if you can't */
	if let Some(&(highest_pos, _)) = digit_map.first()  {
		if highest_pos != digit_map.len() -1 {
			digit_map.retain(|&(pos, _)| pos >= highest_pos);	// Remove all elements wiht position lower, than the highest position
		}
	}
	
	let mut top_two: Vec<(usize, u8)> = digit_map.into_iter().take(2).collect();
    top_two.sort_by(|a, b| a.0.cmp(&b.0));
    
    joltage = top_two[0].1 * 10 + top_two[1].1;
    
	return joltage;
}