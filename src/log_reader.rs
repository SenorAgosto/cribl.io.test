use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::result::Result;
use circular_queue::CircularQueue;

pub fn read_log(file_name : String, num_lines : u64, filter : Option<String> ) -> Result<String, io::Error> {

	let mut lines = CircularQueue::<String>::with_capacity(num_lines as usize);

	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	let pattern = match filter {
		Some(v) => v,
		None => "".to_string(),
	};

	for line_iter in reader.lines() {

		let line = line_iter.unwrap();
		
		if pattern.is_empty() {

			lines.push(line);

		} else {

			if line.contains(&pattern) {
				lines.push(line);
			}	
		}
	}

	let mut r = "".to_string();
	for line in lines.iter() {

		r = r + line + "\n";
	}
	
	Ok(r)
} 
