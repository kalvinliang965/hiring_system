mod applicant;
mod hiring_table;

pub use applicant::Applicant;
pub use hiring_table::HiringTable; 

pub fn run() -> Result<(), String> {
	let applicant = Applicant::from(
		vec!["Facebook".to_string()],
		"Mark Zuck",
		3.99,
		"Harvard",
		vec!["Business Management".to_string()]
	);

	println!("{}", applicant);
	todo!();
	Ok(())
}
