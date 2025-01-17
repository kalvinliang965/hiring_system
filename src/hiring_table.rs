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
		self.data.len()
	}

	fn addApplicant(&mut self, newApplicant: Applicant) {
		self.data.push(newApplicant);
	}

	fn removeApplicant(&mut self, name: &str) {
		self.data.retain(|&applicant| applicant.name != name);
	}

	fn getApplicant(&self, name: String) -> Option<&Applicant> {
		for applicant in self.data {
			if applicant.name == name {
				return Some(applicant)
			}
		}
		None
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

#[cfg(test)]
mod tests {
	
	use super::*;

	#[test]
	fn construct_hiring_table() {
		let hiring_table = Hiring_Table::new();
		assert_eq!(hiring_table.data, Vec::new())
	}

}
