
mod applicant;
mod hiring_table;


pub use applicant::Applicant;
pub use hiring_table::HiringTable; 

use std::io::{self, Write};

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


fn hs_add_applicant() -> Result<(), String> {
	#[cfg(debug_assertions)]
	{
		println!("DEBUG: Add applicant");
	}
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
fn hs_print_list() -> Result<(), String> {
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

	'simulation: loop {
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
				if let Err(err) = hs_add_applicant() {
					return Err(err);
				}
			}
			"R" => {
				if let Err(err) = hs_remove_applicant() {
					return Err(err);
				}
			}
			"G" => { 
				if let Err(err) = hs_get_applicant() {
					return Err(err);
				}
			}
			"P" => { 
				if let Err(err) = hs_print_list() {
					return Err(err);
				}
			}
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
