
use super::applicant::Applicant;

pub struct HiringTable {
	data: Vec<Applicant>,
}

impl HiringTable {
	
	const MAX_SKILLS: u8 = 3;
	const MAX_COMPANIES: u8 = 3;
	const MAX_APPLICATIONS: u8 = 50;
	
	fn new() -> Self {
		HiringTable {
			data: Vec::new()
		}
	}

	fn refine_search(table: Self, company: String, 
		skill: String, college: String, gpa: f64) {
		
		todo!();
	}

	fn size(&self) -> usize {
		self.data.len()
	}

	fn add_applicant(&mut self, new_applicant: Applicant) {
		self.data.push(new_applicant);
	}

	fn remove_applicant(&mut self, name: &str) {
		self.data.retain(|applicant| applicant.get_applicant_name() != name);
	}

	fn get_applicant(&self, name: String) -> Option<&Applicant> {
		for applicant in &self.data {
			if applicant.get_applicant_name() == name {
				return Some(applicant)
			}
		}
		None
	}
	
	fn print_applicant_table() {
		todo!();
	}

}

impl Clone for HiringTable {
	fn clone(&self)->Self {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	
	use super::*;

	#[test]
	fn construct_hiring_table() {
		let hiring_table = HiringTable::new();
		assert_eq!(hiring_table.data, Vec::new())
	}

	#[test]
	fn adding_applicant() {
		let mut hiring_table = HiringTable::new();
		let applicant = Applicant::from(
			vec!["facebook".to_string(), "meta".to_string()],
			"Tommy",
			3.2,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);

		let applicant_clone = applicant.clone();
		hiring_table.add_applicant(applicant);
		assert_eq!(hiring_table.data, vec![applicant_clone]);
	}

}
