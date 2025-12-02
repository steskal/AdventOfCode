use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

/* Test data expected to be in target directory for the build. */
//const DATA_FILE: &str = "./test_data.txt";
const DATA_FILE: &str = "./input.txt";
const WRAP_MAX: u16 = 100;

fn main() {
    
	let starting_position		= 50;
	let mut current_position	= starting_position;
	let mut zeroes_counter		= 0;
	
    let file 	= File::open(DATA_FILE).expect("Failed to open data file.\n");
    let reader	= BufReader::new(file);
    
    for line_result in reader.lines() {
    	let line 						= line_result.unwrap(); // if you can't read line, just end with error
     	let (direction, count) 			= line.split_at(1);
      	let count_parsed: u16			= count.trim().parse().expect(&format!("Cannot parse count {} from isntruction {}{}", count, direction, count));
        
      	match direction {
       		"L"=>{
         		current_position = wrap_sub(current_position as u16, count_parsed); //issue if WRAP_MAX is > 127
         	},
         	"R"=>{
          		current_position = wrap_add(current_position as u16, count_parsed); // issue if WRAP_MAX IS > 127
          	},
           	_ => {
           		eprintln!("Cannot process instruction {}{}", direction, count);
             	exit(1);
            }
       	}
        
        //println!("Current position: {}", current_position);
        if current_position == 0 {
        	zeroes_counter += 1;
        }
    }
    
    println!("Zeroes counter: {}", zeroes_counter);
    
}

/* 
 * Add umbers, if it goes over 100, wrap it from 0 - eg 50 + 55 results in 5.
 * Note this doesnt is only ment to work for values between 0 and WRAP_MAX limit. 
 * This absolutely isn't safe and can go horribly wrong if the WRAP_MAX limit iS >= 127.
 * Make sure that the b value is not highher than WRAP_MAX before passing it to the function.
 * */
fn wrap_add(a: u16, b: u16) -> u8 {
	return ((a + b) % WRAP_MAX) as u8;
}

/* 
 * Subtract wrapping value if overflows 0 - eg 50 - 55 -> 95 
 * Note this is only ment to work for values between 0 and WRAP_MAX limit.
 * This absolutely isn't safe and can go horribly wrong if the WRAP_MAX limit iS >= 127.
 * Make sure that the b value is not highher than WRAP_MAX before passing it to the function.
 * */
fn wrap_sub(a: u16, b: u16) -> u8 {
	
	let b_cleared = b % WRAP_MAX;
	let ret: i16 = a as i16 - b_cleared as i16;
	
	return ((ret + WRAP_MAX as i16) % WRAP_MAX as i16) as u8;
}

