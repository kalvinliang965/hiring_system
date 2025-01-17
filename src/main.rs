use std::process;

fn main() {
	
	if let Err(e) = hiring_system_simulation::run() {
		eprintln!("Application error: {e}");
		process::exit(1);
	}
}
