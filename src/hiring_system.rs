
mod applicant;
mod hiring_table;


pub use applicant::Applicant;
pub use hiring_table::HiringTable; 
use std::io::{self, Write};
use std::process;

fn menu() {
	println!("(A) Add Applicant");
	println!("(R) Remove Applicant");
	println!("(G) Get Applicant");
	println!("(P) Print List");
	println!("(RS) Refine Search");
	println!("(S) Size");
	println!("(D) Backup");
	println!("(CS) Compare Backup");
	println!("(RB) Revert Backup");
	println!("(Q) Quit");
}


fn read_line(prompt: &str) -> Result<String, String> {
	print!("{}: ", prompt);
	io::stdout().flush().map_err(|e| e.to_string())?;
	let mut input = String::new();
	io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
	Ok(input.trim().to_string())
}

fn read_float(prompt: &str) -> Result<f64, String> {
	print!("{}: ", prompt);
	let mut input = String::new();
	io::stdout().flush().map_err(|e| e.to_string())?;
	io::stdin().read_line(&mut input).map_err(|e| e.to_string())?; 
	match input.trim().parse::<f64>() {
		Ok(value) => Ok(value),
		Err(_) => Err("Invalid number format. Please enter a valid float".to_string()),
	}
}

fn hs_add_applicant(hs_table: &mut HiringTable) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Add applicant");
	}
	
	if hs_table.size() >= HiringTable::MAX_APPLICANTS.into() {
		return Err("The maximum number of applicants has been reached".to_string());
	}

	// applicant name input
	let applicant_name = match read_line("Enter Applicant Name") {
		Ok(name) => name,
		Err(err) => { 
			return Err(format!("Error in reading applicant name: {}", err));
		}
	};

	let applicant_gpa = match read_float("Enter Applicant GPA") {
		Ok(gpa) => gpa,
		Err(err) => {
			return Err(format!("Error in reading applicant gpa: {}", err));
		}
	};
	
	// college input
	let applicant_college = match read_line("Enter applicant college") {
		Ok(college) => college,
		Err(err) => {
			return Err(format!("Error in reading applicant colelge: {}", err));
		}
	};

	let mut num_companies = HiringTable::MAX_COMPANIES;
	let mut company_name = Vec::new();
	while num_companies > 0 {
		match read_line(&format!("Enter up to {} Companies", num_companies)) {
			Ok(company) => if company.len() > 0 {company_name.push(company)} else {},
			Err(err) => {
				return Err(format!("Error in reading company name: {}", err));
			}
		};
		num_companies -= 1;
	}
	
	let mut num_skills = HiringTable::MAX_SKILLS;
	let mut applicant_skills = Vec::new();
	while num_skills > 0 {
		match read_line(&format!("Enter up to {} Skills", num_skills)) {
			Ok(skill) => if skill.len() > 0 {applicant_skills.push(skill)} else {},
			Err(err) => {
				return Err(format!("Error in reading applicant skill: {}", err));
			}
		};
		num_skills -= 1;
	}
	
	let applicant = Applicant::from(
		company_name,
		&applicant_name,
		applicant_gpa,
		&applicant_college,
		applicant_skills,
	);

	hs_table.add_applicant(applicant);

	Ok(())
}

fn hs_remove_applicant() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: remove_applicant")
	}
	Ok(())
}
fn hs_get_applicant() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Get applicant");
	}
	Ok(())
}
fn hs_print_list(hs_table: &HiringTable) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Print List");
	}
	Ok(())
}
fn hs_refine_search() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Refine Search");
	}
	Ok(())
}
fn hs_size() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Size");
	}
	Ok(())
}
fn hs_backup() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Backup");
	}
	Ok(())
}
fn hs_compare_backup() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Compare Backup");
	}
	Ok(())
}
fn hs_revert_backup() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: revert backup");
	}
	Ok(())
}
pub fn main() -> Result<(), String> {
	
	let mut hs_table = HiringTable::new();
	// contain different version of hs_table
	let mut backup_table: Vec<HiringTable>  = Vec::new();
	// version of current hiring table
	let mut version = 0;

	'simulation: loop {
		
		menu();

		let mut command = String::new();
		print!("Please enter a command: ");
		io::stdout().flush().unwrap();

		if let Err(err) = io::stdin().read_line(&mut command) {
			eprintln!("Error reading input: {}", err);
			return Err(err.to_string());
		}
		
		let uppercase_command = command.trim().to_uppercase();
		let parse_command = uppercase_command.as_str();

		match parse_command {
			"A" => {
				if let Err(err) = hs_add_applicant(&mut hs_table) {
					eprintln!(err);
				}
			}
			"R" => {
				if let Err(err) = hs_remove_applicant() {
					eprintln!(err);
				}
			}
			"G" => { 
				if let Err(err) = hs_get_applicant() {
					return Err(err);
				}
			}
			"P" => hs_table.print_applicant_table();
			"RS" => {
				if let Err(err) = hs_refine_search() {
					return Err(err);
				}
			}
			"S" => {
				if let Err(err) = hs_size() {
					return Err(err);
				}
			}
			"D" => {
				if let Err(err) = hs_backup() {
					return Err(err);
				}
			}
			"CB" => {
				if let Err(err) = hs_compare_backup() {
					return Err(err);
				}
			}
			"RB" => {
				if let Err(err) = hs_revert_backup() {
					return Err(err);
				}
			}
			"Q" => {
				break 'simulation;
			} 
			_  => {
				println!("Command enter: {parse_command}");
				println!("Invalid command");
			}
		}
	}
	Ok(())
}
