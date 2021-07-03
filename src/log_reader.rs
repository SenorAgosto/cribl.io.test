use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
use std::result::Result;

pub fn read_log(file_name : String, num_lines : u64, filter : Option<String> ) -> Result<String, io::Error> {

	let mut lines : Vec<String> = Vec::with_capacity(num_lines as usize); 
	lines.resize(num_lines as usize, "".to_string());

	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
	}

	Ok(lines.join("\n"))
} 

