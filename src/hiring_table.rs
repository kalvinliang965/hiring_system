mod applicant;

use applicant::Applicant;

struct Hiring_Table {
	data: Vec<Applicant>,
}

impl Hiring_Table {
	
	const MAX_SKILLS: u8,
	const MAX_COMPANIES: u8,
	const MAX_APPLICATIONS: u8,
	
	fn new() -> Self {
		Hiring_Table {
			data: Vec::new()
		}
	}

	fn refineSearch(table: Self, company: String, 
		skill: String, college: String, gpa: f64) {
		
		todo!();
	}

	fn size(&self) -> u64 {
		todo!();
	}

	fn addApplicant(newApplicant: Applicant) {
		todo!();
	}

	fn removeApplicant(name: String) {
		todo!();
	}

	fn getApplicant(name: String) -> &Applicant {
		todo!();
	}
	
	fn printApplicantTable() {
		todo!();
	}

}

impl Clone for Hiring_Table {
	fn clone() {
		todo!();
	}
}
