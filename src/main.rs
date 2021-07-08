#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;

mod log_path;
use log_path::LogPath;

mod log_reader;
use log_reader::read_log;
use log_reader::read_log_with_filter;

#[get("/log/<log_name>/<num_lines>")]
fn get_log(logs : State<LogPath>, log_name : String, num_lines : usize) -> String {
	
	let mut path = logs.path.clone();
	path.push_str("/");
	path = path + &log_name;

	match read_log(&path, num_lines) {
		Ok(r) => r,
		Err(e) => panic!("{}", e),
	}
}

#[get("/filtered_log/<log_name>/<filter>/<num_lines>")]
fn get_filtered_log(logs : State<LogPath>, log_name : String, filter : String, num_lines : usize) -> String {

	let mut path = logs.path.clone();
	path.push_str("/");
	path = path + &log_name;

	match read_log_with_filter(&path, filter, num_lines) {
		Ok(r) => r,
		Err(e) => panic!("{}", e),
	}
}

fn main() {

	rocket::ignite()
		.manage(log_path::load_path())
		.mount("/", routes![get_log,get_filtered_log]).launch();
}

