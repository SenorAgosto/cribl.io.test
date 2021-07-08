use std::fs::File;
use std::io::{Error, ErrorKind};
use std::result::Result;
use memmap::MmapOptions;

fn line_from_mmap(slice : &[u8]) -> Result<String, std::io::Error> {

	match std::str::from_utf8(slice).to_owned() {
		Ok(v) => return Ok(v.to_string()),
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

pub fn read_log(file_name : String, num_lines : u64, filter : Option<String> ) -> Result<String, std::io::Error> {

	let num_lines : usize = num_lines as usize;
	let mut lines = Vec::<String>::with_capacity(num_lines as usize);

	let file = File::open(file_name)?;
	let mmap = unsafe { MmapOptions::new().map(&file)? };

	let pattern = match filter {
		Some(v) => v,
		None => "".to_string(),
	};

	let mut collected_lines : usize = 0;
	let mut prev_spot : usize = mmap.len() - 1;
	let mut spot : usize = mmap.len() - 2; // skip the last character of the buffer in case it's '\n'

	while spot > 0 && collected_lines < num_lines {	

		if mmap[spot] == b'\n' {

			let line = line_from_mmap(&mmap[spot + 1..prev_spot])?;

			if pattern.is_empty() {
				lines.push(line.to_string()); 
				collected_lines += 1;
			} else if line.contains(&pattern) {
				lines.push(line.to_string());
				collected_lines += 1;
			}

			prev_spot = spot;
		}

		spot -= 1;
	};

	// grab the first line of the log if we haven't collect enough lines
	if spot == 0 && collected_lines < num_lines {
		let line = line_from_mmap(&mmap[spot..prev_spot])?;	

		if pattern.is_empty() {
			lines.push(line.to_string());
		} else if line.contains(&pattern) {
			lines.push(line.to_string());
			
		}
	}

	let capacity = mmap.len() - spot + 1;
	collect_lines(&lines, capacity)
} 
