#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;

mod log_path;
use log_path::LogPath;

mod log_reader;
use log_reader::read_log;

#[get("/log/<log_name>/<num_lines>")]
fn get_log(logs : State<LogPath>, log_name : String, num_lines : u64) -> String {
	format!("log_path: {}, log_name: {}, num_lines: {}", logs.path, log_name, num_lines)
}

#[get("/filtered_log/<log_name>/<filter>/<num_lines>")]
fn get_filtered_log(logs : State<LogPath>, log_name : String, filter : String, num_lines : u64) -> String {
	format!("log_path: {}, log_name: {}, filter: {},  num_lines: {}", logs.path, log_name, filter, num_lines)
}

fn main() {

	rocket::ignite()
		.manage(log_path::load_path())
		.mount("/", routes![get_log,get_filtered_log]).launch();
}

