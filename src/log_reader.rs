use std::fs::File;
use std::result::Result;
use memmap::MmapOptions;

mod capture_policy;
use capture_policy::{CapturePolicy, FilteredCapture, UnfilteredCapture};

fn read_log_with_capture_policy<L : CapturePolicy>(capture : &mut L, file_name : &String) -> Result<String, std::io::Error> {

	let file = File::open(&file_name)?;
	let mmap = unsafe { MmapOptions::new().map(&file)? };

	// TODO: handle the case the log is only 2 bytes
	assert!(mmap.len() >= 2);

	let mut prev_spot : usize = mmap.len() - 1;
	let mut spot : usize = mmap.len() - 2; // skip the last char in the buffer in case it's '\n'

	while spot > 0 && capture.continue_capture() {

		if mmap[spot] == b'\n' {
			let line = &mmap[spot + 1..prev_spot];

			capture.capture_line(line);
			prev_spot = spot;
		}

		spot -= 1;
	};
	
	// grab the first line of the log if we haven't collect enough lines
	if spot == 0 && capture.continue_capture() {

		let line = &mmap[spot..prev_spot];
		capture.capture_line(line);
	}

	let capacity = mmap.len() - spot + 1;
	capture.collect_lines(capacity)
}

pub fn read_log(file_name : &String, num_lines : usize) -> Result<String, std::io::Error> {

	let mut capture = UnfilteredCapture::with_capacity(num_lines);
	read_log_with_capture_policy(&mut capture, &file_name)
}

pub fn read_log_with_filter(file_name : &String, filter : String, num_lines : usize) -> Result<String, std::io::Error> {

	let f = filter.as_bytes();
	let mut capture = FilteredCapture::with_capacity(&f, num_lines);
	read_log_with_capture_policy(&mut capture, &file_name)
}

