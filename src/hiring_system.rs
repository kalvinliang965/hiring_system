
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
	println!("(CB) Compare Backup");
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
		Ok(value) => {
			if value < 0.0 {
				return Err("Please enter a positive value".to_string());
			} 
			Ok(value)
		}
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
		Ok(name) => {
			if name.len() == 0 {
				return Err("Please enter a valid applicant name".to_string());
			} else {
				name
			}
		}
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
		Ok(college_name) => college_name,
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

fn hs_remove_applicant(hs_table: &mut HiringTable) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: remove_applicant")
	}
	
	if hs_table.size() == 0 {
		return Err("Hiring System currently has no applicant".to_string());
	}
	
	let applicant_name = match read_line("Enter applicant name") {
		Ok(name) => name,
		Err(err) => {
			return Err(format!("Error in getting applicant name: {}", err));
		}
	};
	match hs_table.remove_applicant(&applicant_name) {
		Some(applicant) => {
			#[cfg(debug_assertions)]
			{
				assert_eq!(applicant_name, applicant.get_applicant_name());
			};
			println!("Applicant {} has been successfully removed form the hiring system.", applicant_name);
		}
		None => println!("Applicant {} does in exist in the Hiring System", applicant_name),
	}
	Ok(())
}
fn hs_get_applicant(hs_table: &HiringTable) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Get applicant");
	}
	
	let applicant_name = match read_line("Enter Applicant Name: ") {
		Ok(name) => name,
		Err(err) => {
			return Err(format!("Error in reading applicant name: {}", err));
		}
	};

	match hs_table.get_applicant(&applicant_name) {
		None => {
			println!("Applicant does not exist");
		},
		Some(applicant) => {

			println!("Applicant Name: {}", applicant.get_applicant_name());
			println!("Applicant Applying From: {}", applicant.get_company_name().join(", "));
			println!("Applicant GPA: {}", applicant.get_applicant_gpa());
			println!("Applicant College: {}", applicant.get_applicant_college());
			println!("Applicant SKills: {}", applicant.get_applicant_skills().join(", "));
		},
	};
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

	let company_filter = match read_line("Enter a company to filter for") {
		Ok(company_name) => Some(company_name),
		Err(err) => {
			return Err(format!("Error in retrieving company name: {}", err));
		}
	};
	let skill_filter = match read_line("Enter a skill to filter for") {
		Ok(skill) => Some(skill),
		Err(err) => {
			return Err(format!("Error in retrieving skill: {}", err));
		}
	};
	let college_filter = match read_line("Enter a college to filter for") {
		Ok(college_name) => Some(college_name),
		Err(err) => {
			return Err(format!("Error in retrieving college name: {}", err));
		}
	};
	let min_gpa_filter = match read_line("Enter the minimum GPA  to filter for") {
		Ok(min_gpa) => Some(min_gpa),
		Err(err) => {
			return Err(format!("Error in retrieving min_gpa: {}", err));
		}
	};

	Ok(())
}
fn hs_size(hs_table: &HiringTable) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Size");
	}
	
	if hs_table.size() == 1 {
		println!("There are {} applicant", hs_table.size());
	} else {
		println!("There are {} applicants", hs_table.size()); 
	}

	Ok(())
}
fn hs_backup(hs_table: &HiringTable, backup_table: &mut Vec<HiringTable>) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Backup");
	}

	let dup = hs_table.clone();
	backup_table.push(dup);
	Ok(())
}

fn hs_print_backup_table(backup_table: &Vec<HiringTable>) {
	println!();
	for (version, table) in backup_table.iter().enumerate() {
		println!("Version: {}", version);
		table.print_applicant_table();
		println!();
	}
}
fn hs_compare_backup(hs_table: &HiringTable, backup_table: &Vec<HiringTable>) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Compare Backup");
	}
	
	if backup_table.len() == 0 {
		println!("There are currently no backup table");
		return Ok(());
	}

	hs_print_backup_table(backup_table);

	let version: usize = match read_float("Please enter the version you want to compare") {
		Ok(v) => v.round() as usize,
		Err(err) => {
			return Err(format!("Error reading version: {}", err));
		},
	};
	
	if backup_table[version] == *hs_table {
		println!("Current list is the same as the backup copy.");
	} else {
		println!("Currnet list is not the same as the backup copy.");
	}
	
	Ok(())
}
fn hs_revert_backup(hs_table: &mut HiringTable, backup_table: &Vec<HiringTable>) -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: revert backup");
	}

	if backup_table.len() == 0 {
		println!("There are currently no backup table");
		return Ok(());
	}

	hs_print_backup_table(backup_table);
	let version: usize = match read_float("Please enter the version you want to revert back to") {
		Ok(v) => v.round() as usize,
		Err(err) => {
			return Err(format!("Error reading version: {}", err));
		},
	};

	*hs_table = backup_table[version].clone();
	Ok(())
}
pub fn main() -> Result<(), String> {
	
	let mut hs_table = HiringTable::new();
	// contain different version of hs_table
	let mut backup_table: Vec<HiringTable>  = Vec::new();

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

		println!();
		match parse_command {
			"A" => {
				if let Err(err) = hs_add_applicant(&mut hs_table) {
					eprintln!("{}", err);
				}
			}
			"R" => {
				if let Err(err) = hs_remove_applicant(&mut hs_table) {
					eprintln!("{}", err);
				}
			}
			"G" => { 
				if let Err(err) = hs_get_applicant(&hs_table) {
					eprintln!("{}", err);
				}
			}
			"P" => hs_table.print_applicant_table(),
			"RS" => {
				if let Err(err) = hs_refine_search() {
					return Err(err);
				}
			}
			"S" => {
				if let Err(err) = hs_size(&hs_table) {
					eprintln!("{}", err);	
				}
			}
			"D" => {
				if let Err(err) = hs_backup(&hs_table, &mut backup_table) {
					eprintln!("{}", err);
				}
			}
			"CB" => {
				if let Err(err) = hs_compare_backup(&hs_table, &backup_table) {
					eprintln!("{}", err);
				}
			}
			"RB" => {
				if let Err(err) = hs_revert_backup(&mut hs_table, &backup_table) {
					eprintln!("{}", err);	
				}
			}
			"Q" => 	break 'simulation,
			_  => {
				println!("Command enter: {parse_command}");
				println!("Invalid command");
			}
			
		}

		println!();
	}
	Ok(())
}
