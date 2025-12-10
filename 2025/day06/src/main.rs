use std::fs::File;
use std::io::{BufReader, BufRead};

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	
	let file 			= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader 			= BufReader::new(file);
	
	let mut first_line 						= true;
	let mut instructions: Vec<char> 		= Vec::new();
	let mut problem_numbers: Vec<Vec<u64>> 	= Vec::new();
	
	let mut part_one: u64 = 0;
	
	for line_result in reader.lines() {
		
		let line 	= line_result.expect("Failed to read the line.");
		
		if line.contains("*") || line.contains("+") { // For the last instruction line
			
			instructions = line.split_whitespace().filter_map(|chunk| chunk.chars().next()).collect();
			continue;
		}
		
		let numbers = split_to_nums_by_whitespace(&line);
		
		if first_line { // For the first line, create the vectors
			for _ in 0..numbers.len() {
				problem_numbers.push(Vec::new());
				first_line = false;
			}
		}
		for idx in 0..numbers.len() { // For every line except the last one
			problem_numbers[idx].push(numbers[idx] as u64);
		}
	}
	
	/* Calculate result for part one */
	for idx in 0..problem_numbers.len() {
		match instructions[idx]{
			'+' => {
				part_one += (problem_numbers[idx].iter()).sum::<u64>();
			}, 
			'*' => {
				part_one += problem_numbers[idx].iter().product::<u64>();
			},
			_ => {
				panic!("Unexpected instruction.");
			}
		}
	}
	
	// RESULT
	println!("Part One: {}", part_one);
}

fn split_to_nums_by_whitespace(line: &str) -> Vec<u32> {
	let mut nums = Vec::new();
	
	for num_str in line.split_whitespace() {
		if let Ok(num) = num_str.parse::<u32>() {
			nums.push(num);
		}
	}
	nums
}