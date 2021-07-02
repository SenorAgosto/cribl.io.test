#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;
mod LogPath;

#[get("/log/<log_name>/<num_lines>")]
fn get_log(log_path : State<LogPath::LogPath>, log_name : String, num_lines : u64) -> String {
	format!("log_name: {}, num_lines: {}", log_name, num_lines)
}

#[get("/filtered_log/<log_name>/<filter>/<num_lines>")]
fn get_filtered_log(log_path : State<LogPath::LogPath>, log_name : String, filter : String, num_lines : u64) -> String {
	format!("log_name: {}, filter: {},  num_lines: {}", log_name, filter, num_lines)
}

fn main() {

	rocket::ignite()
		.manage(LogPath::load_path())
		.mount("/", routes![get_log,get_filtered_log]).launch();
}

