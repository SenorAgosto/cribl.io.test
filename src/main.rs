#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/log/<log_name>/<num_lines>")]
fn get_log(log_name : String, num_lines : u64) -> String {
	format!("log_name: {}, num_lines: {}", log_name, num_lines)
}

fn main() {
	rocket::ignite().mount("/", routes![get_log]).launch();
}
