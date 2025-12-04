use std::fs::File;
use std::io::{BufRead, BufReader};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file 	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader 	= BufReader::new(file);
	
	let mut	total_output_part1:u64	= 0;
	let mut total_output_part2:u128 = 0;
	
	for line_result in reader.lines() {
		let line 	= line_result.unwrap();	// will panic, if the line cannot be read
		
		total_output_part1 += largest_joltage_2(&line) as u64;
		total_output_part2 += largest_joltage_n(&line,0, 12).expect("Function failed");
		
	}
	println!("Part 1: {}", total_output_part1);
	println!("Part 2: {}", total_output_part2);
}

/* Take a string and find lagest possible 2 digit number you can get in the 
 * string without reordering eg:
 * 		In 818181911112111, the largest number you can produce is 92
 * 		
 * */
fn largest_joltage_2 (input: &str) -> u8 {
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
    
    let joltage = top_two[0].1 * 10 + top_two[1].1;
    
	return joltage;
}

/* Take a string and find largest possible n digits number you can get 
 * in the string without reordering. Eg if n = 12
 * 		In 818181911112111, the highest possible nuber is 888911112111
 * 
 * This is recursive function.
 */
fn largest_joltage_n (input: &str, from: usize, n: u8) -> Option<u128> {
	if n as usize > input.len() || from as usize > input.len() - 1 {
		return None;	
	}
	
	let mut number:u128;
	/* Find the max in range */
	let digits: Vec<u8> = input.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect();	// Convert to numbers
	let slice 			= digits.get(from as usize..=(digits.len() - n as usize))?;					// Get relevant slice - mins if the n is 12 digits long, I can only search for highest digit in pos 0 - len - n (exluced)
	let max_val 		= slice.iter().max()?;													//Get the max value
	let max_input_pos	= slice.iter().position(|&v| v == *max_val)?;
	
	number = *max_val as u128 * 10u128.pow((n - 1) as u32);
	
	if n > 1 {
		number += largest_joltage_n(input, max_input_pos + from + 1, n - 1).unwrap(); 
	}
	
	return Some(number);
}