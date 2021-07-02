use std::env;

pub struct LogPath {
	pub path : String,
}

// read the log path from command line arguments, default to /var/log if not present
pub fn load_path() -> LogPath {

	let args : Vec<String> = env::args().collect();
	if args.len() < 2 {
		LogPath { path : "/var/log".to_string() }
	} else {
		LogPath { path : args[1].to_string() }
	}
}
