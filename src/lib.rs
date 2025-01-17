mod hiring_system;

pub use hiring_system::Applicant;
pub use hiring_system::HiringTable; 

pub fn run() -> Result<(), String> {
	if let Err(e) = hiring_system::main() {
		return Err(e);
	}
	Ok(())
}
