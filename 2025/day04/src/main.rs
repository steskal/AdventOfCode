use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

/* Data files are expected in the <build_target>/src_data/ directory */
//const DATA_FILE: &str = "./src_data/test_data.txt";
const DATA_FILE: &str = "./src_data/input.txt";

fn main() {
	let file 	= File::open(DATA_FILE).expect("Failed to open datafile.");
	let reader 	= BufReader::new(file);
	
	let mut rows: Vec<Vec<u8>>	= Vec::new();
	let mut accessible_1: u32	= 0;
	
	for line_result in reader.lines() {
		let line 	= line_result.unwrap();
		rows.push(line.chars()
						.map(|c| match c {
							'.' => 0,
							'@' => 1,
							_ => 0
			}).collect()); //convert it to field of 0 and 1 where 1 represents a roll
	}
	
	for y in 0..rows.len() { // go through all rolls
		for x in 0..rows.get(y).unwrap().len() {
			if rows[y][x] != 1 {
				continue;	//only rolls, don't check for empty spaces.
			}
			if is_accessible(&rows, x, y) {
				accessible_1 += 1;
			}
		}
	}
	
	println!("Accessible rolls part 1: {}", accessible_1);
	
}


/* Return true if roll is accessible - there are less than 4 rolls in the eight 
 * adjacent positions. 
 * ONLY USE ON pos_x pos_y THAT IS A ROLL!
 * */
 
 fn is_accessible(rows: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize) -> bool{
 	let mut count 	= 0;
  	
  	let pos_x_top = cmp::min(pos_x + 1, rows.first().unwrap().len() -1 );
    let pos_y_top = cmp::min(pos_y + 1, rows.len() - 1);
  
  	for y in pos_y.saturating_sub(1)..=pos_y_top {
   
   		let slice = rows.get(y).unwrap().get(pos_x.saturating_sub(1)..=pos_x_top).unwrap();
     	
      	count +=slice.iter().sum::<u8>();
       	if y == pos_y {
        	count -= 1;	//because it counts also the current role for which we are only searching for neighbours
        }
   }
    
  	return count < 4;
 }