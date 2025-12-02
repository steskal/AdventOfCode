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
	let mut zeroes_counter:u32		= 0;
	
    let file 	= File::open(DATA_FILE).expect("Failed to open data file.\n");
    let reader	= BufReader::new(file);
    
    for line_result in reader.lines() {
    	let line 						= line_result.unwrap(); // if you can't read line, just end with error
     	let (direction, count) 			= line.split_at(1);
      	let count_parsed: u16			= count.trim().parse().expect(&format!("Cannot parse count {} from isntruction {}{}", count, direction, count));
       
      	match direction {
       
       		"L"=>{
         		let simply_substracted: i16 = current_position as i16 - count_parsed as i16;
         		if current_position <= count_parsed {
           			zeroes_counter += simply_substracted.abs() as u32 / WRAP_MAX as u32;
              		/* if current position is > 0 - you pas 0 one more time:
                 	 * eg: 50 - 150 = -100 -> you've clicked on the 0 twice if roter is range 0 - 99
                   	 * where as if you start on 0 and substract 100 - you'd click on 0 only once.
                     */
                     
              		if current_position != 0 {
                		zeroes_counter += 1;
               		}
           		}
         		current_position = wrap_sub(current_position as u16, count_parsed);
         	},
          
         	"R"=>{
          		let simply_added = current_position + count_parsed;
          		if simply_added >= WRAP_MAX {
          			zeroes_counter += (simply_added/ WRAP_MAX) as u32;
            	}
             	
          		current_position = wrap_add(current_position as u16, count_parsed);
          	},
           
           	_ => {
           		eprintln!("Cannot process instruction {}{}", direction, count);
             	exit(1);
            }
       	}
    }
    
    println!("Zeroes counter: {}", zeroes_counter);
    
}

/* 
 * Add umbers, if it goes over 100, wrap it from 0 - eg 50 + 55 results in 5.
 * Note this doesnt is only ment to work for values between 0 and WRAP_MAX limit. 
 * */
fn wrap_add(a: u16, b: u16) -> u16 {
	return (a + b) % WRAP_MAX;
}

/* 
 * Subtract wrapping value if overflows 0 - eg 50 - 55 -> 95 
 * Note this is only ment to work for values between 0 and WRAP_MAX limit.
 * */
fn wrap_sub(a: u16, b: u16) -> u16 {
	
	let b_cleared = b % WRAP_MAX;
	let ret: i16 = a as i16 - b_cleared as i16;
	
	return ((ret + WRAP_MAX as i16) % WRAP_MAX as i16) as u16;
}

