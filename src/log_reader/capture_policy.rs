use needle::BoyerMoore;
use std::io::{Error, ErrorKind};

fn collect(lines : &Vec::<String>, capacity : usize) -> Result<String, std::io::Error> {

	let mut r = String::with_capacity(capacity);
	for line in lines.iter() {
		r += line;
		r.push_str("\n");
	}

	Ok(r)
}

fn line_from_bytes(slice : &[u8]) -> Result<String, std::io::Error> {

	match std::str::from_utf8(slice).to_owned() {
		Ok(v)	=> return Ok(v.to_string()),
		Err(_e) => return Err(Error::new(ErrorKind::Other, "utf-8 conversion error")),
	}
}

pub trait CapturePolicy {
	fn continue_capture(&self) -> bool;
	fn capture_line(&mut self, line : &[u8]);
	fn collect_lines(&self, capacity : usize) -> Result<String, std::io::Error>;
}

pub struct UnfilteredCapture {
	max_lines : usize,
	lines : Vec<String>,
}

impl UnfilteredCapture {
	pub fn with_capacity(capacity : usize) -> UnfilteredCapture {
	
		UnfilteredCapture {
			max_lines : capacity,
			lines : Vec::<String>::with_capacity(capacity),
		}	
	}
}

impl CapturePolicy for UnfilteredCapture {

	fn capture_line(&mut self, line : &[u8]) {
		let s = line_from_bytes(line).unwrap();
		self.lines.push(s);
	} 

	fn collect_lines(&self, capacity : usize) -> Result<String, std::io::Error> {
		collect(&self.lines, capacity)
	}

	fn continue_capture(&self) -> bool {
		self.lines.len() < self.max_lines
	}
}

pub struct FilteredCapture<'a> {

	max_lines : usize,
	pattern : BoyerMoore<'a, [u8]>,
	lines : Vec<String>,
}

impl FilteredCapture<'_> {
	pub fn with_capacity<'a>(filter : &'a [u8], capacity : usize) -> FilteredCapture {

		FilteredCapture {
			max_lines : capacity,
			pattern : BoyerMoore::new(&filter),
			lines : Vec::<String>::with_capacity(capacity),
		}
	}

	fn is_match(&self, line : &[u8]) -> bool {

		if line.len() == 0 {
			return false;
		}

		self.pattern.find_in(line).next().is_some()
	}
}

impl CapturePolicy for FilteredCapture<'_> {

	fn capture_line(&mut self, line : &[u8] ) {

		if self.is_match(line) {

			let s = line_from_bytes(line).unwrap();
			self.lines.push(s);
		}
	}

	fn collect_lines(&self, capacity : usize) -> Result<String, std::io::Error> {
		collect(&self.lines, capacity)
	}

	fn continue_capture(&self) -> bool {
		self.lines.len() < self.max_lines
	}
}
