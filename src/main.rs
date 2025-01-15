use std::process;

fn main() {
	
	if let Err(e) = hiring_system::run() {
		eprintln!("Application error: {e}");
		process::exit(1);
	}
}
