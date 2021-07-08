use needle::BoyerMoore;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::result::Result;
use memmap::MmapOptions;

fn capture_line(pattern : &BoyerMoore<[u8]>, line : &[u8]) -> bool {

	if line.len() == 0 {
		return false;
	}

	pattern.find_in(line).next().is_some()
}


fn line_from_bytes(slice : &[u8]) -> Result<String, std::io::Error> {
	
	match std::str::from_utf8(slice).to_owned() {
		Ok(v)   => return Ok(v.to_string()),
		Err(_e) => return Err(Error::new(ErrorKind::Other, "utf-8 conversion error")),
	}
}

fn collect_lines(lines : &Vec::<String>, capacity : usize) -> Result<String, std::io::Error> {

	let mut r = String::with_capacity(capacity);
	for line in lines.iter() {

		r += line;
		r.push_str("\n");
	} 

	Ok(r)
}

// TODO: read_log_without_filter() and read_log_with_filter() could probably be 
// a function template
fn read_log_without_filter(file_name : &String, num_lines : usize) -> Result<String, std::io::Error> {

	let mut lines = Vec::<String>::with_capacity(num_lines);
	let file = File::open(&file_name)?;
	let mmap = unsafe { MmapOptions::new().map(&file)? };

	// TODO: handle the case the log is only 2 bytes
	assert!(mmap.len() >= 2);

	let mut collected_lines : usize = 0;
	let mut prev_spot : usize = mmap.len() - 1;
	let mut spot : usize = mmap.len() - 2; // skip the last char in the buffer in case it's '\n'

	while spot > 0 && collected_lines < num_lines {

		if mmap[spot] == b'\n' {
			let line = &mmap[spot + 1..prev_spot];
			let s = line_from_bytes(line)?;

			lines.push(s);
			collected_lines += 1;

			prev_spot = spot;
		}

		spot -= 1;
	};
	
	// grab the first line of the log if we haven't collect enough lines
	if spot == 0 && collected_lines < num_lines {

		let line = &mmap[spot..prev_spot];
		let s = line_from_bytes(line)?;
		
		lines.push(s);
	}

	let capacity = mmap.len() - spot + 1;
	collect_lines(&lines, capacity)
}

fn read_log_with_filter(file_name : &String, filter : &String, num_lines : usize ) -> Result<String, std::io::Error> {
	
	let mut lines = Vec::<String>::with_capacity(num_lines);
	let file = File::open(&file_name)?;
	let mmap = unsafe { MmapOptions::new().map(&file)? };

	// TODO: handle the case the log is only 2 bytes
	assert!(mmap.len() >= 2);

	let pattern = BoyerMoore::new(filter.as_str().as_bytes());

	let mut collected_lines : usize = 0;
	let mut prev_spot : usize = mmap.len() - 1;
	let mut spot : usize = mmap.len() - 2;

	while spot > 0 && collected_lines < num_lines {
		
		if mmap[spot] == b'\n' {
			let line = &mmap[spot + 1..prev_spot];

			if capture_line(&pattern, &line) {
				let s = line_from_bytes(line)?;
				lines.push(s);

				collected_lines += 1;
			}

			prev_spot = spot;
		}

		spot -= 1;
	};
	
	// grab the first line of the log if we haven't collect enough lines
	if spot == 0 && collected_lines < num_lines {

		let line = &mmap[spot..prev_spot];

		if capture_line(&pattern, &line) {
			let s = line_from_bytes(line)?;
			lines.push(s);
		}
	}

	let capacity = mmap.len() - spot + 1;
	collect_lines(&lines, capacity)
}

pub fn read_log(file_name : String, num_lines : u64, filter : Option<String> ) -> Result<String, std::io::Error> {

	let num_lines : usize = num_lines as usize;

	match filter {
		Some(v)  => read_log_with_filter(&file_name, &v, num_lines),
		None	 => read_log_without_filter(&file_name, num_lines),
	}
} 
