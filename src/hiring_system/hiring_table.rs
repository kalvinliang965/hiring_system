use super::applicant::Applicant;

#[derive(PartialEq)]
pub struct HiringTable {
	data: Vec<Applicant>,
}

impl HiringTable {
	
	pub const MAX_SKILLS: u8 = 3;
	pub const MAX_COMPANIES: u8 = 3;
	pub const MAX_APPLICANTS: u8 = 50;
	
	pub fn new() -> Self {
		HiringTable {
			data: Vec::new()
		}
	}


	
	fn filter_applicant(table: &Self, company: Option<String>,
		skill: Option<String>, college: Option<String>, gpa: Option<f64>) -> Vec<&Applicant> {
		
		let company_cond = |applicant: &Applicant| {
			match &company {
				None => true,
				Some(company) => applicant.get_company_name().contains(&company),
			}
		};

		let skill_cond = |applicant: &Applicant| {
			match &skill {
				None => true,
				Some(skill) => applicant.get_applicant_skills().contains(&skill),
			}
		};
		let college_cond = |applicant: &Applicant| {
			match &college {
				None => true,
				Some(college) => applicant.get_applicant_college() == college,
			}
		};
		let gpa_cond = |applicant: &Applicant| {
			match gpa {
				None => true,
				Some(gpa) => applicant.get_applicant_gpa() == gpa,
			}
		};
		
		table.data.iter()
			.filter(|applicant| {
				company_cond(applicant) &&
				skill_cond(applicant) &&
				college_cond(applicant) &&
				gpa_cond(applicant)
			}).collect::<Vec<&Applicant>>() 
	}

	pub fn refine_search(table: &Self, company: String, 
		skill: String, college: String, gpa: f64) {

		HiringTable::print_table_header();
		
	
		
		todo!();
	}

	pub fn size(&self) -> usize {
		self.data.len()
	}

	pub fn add_applicant(&mut self, new_applicant: Applicant)  {
		if new_applicant.get_applicant_skills().len() > HiringTable::MAX_SKILLS.into() {
			panic!("add_applicant: The applicant has exceeded the maximum number of skills allowed");
		}

		if new_applicant.get_company_name().len() > HiringTable::MAX_COMPANIES.into() {
			panic!("add_applicant: The applicant has exceed the maximum number of companies allowed");
		}

		if self.size() > HiringTable::MAX_APPLICANTS.into() {
			panic!("add_applicant: The maximum number of applicants has been reached");
		}
		
		self.data.push(new_applicant);
	}

	pub fn remove_applicant(&mut self, name: &str) -> Option<Applicant>{
		if self.size() == 0 {
			panic!("remove_applicant: currently has no applicant");
		}

		for (index, applicant) in self.data.iter().enumerate() {
			if applicant.get_applicant_name() == name {
				return Some(self.data.remove(index));

			}			
		}
		None
	}

	pub fn get_applicant(&self, name: String) -> Option<&Applicant> {
		for applicant in &self.data {
			if applicant.get_applicant_name() == name {
				return Some(applicant)
			}
		}
		None
	}
	fn print_table_header() {
		println!("{:<40} {:<20} {:<6} {:<20} {:<40}",
			"Company Name",
			"Applicant",
			"GPA",
			"College",
			"Skills",
		);
		println!("{}", "_".repeat(40 + 20 + 6 + 20 + 40));
	}	
	pub fn print_applicant_table(&self) {
		HiringTable::print_table_header();	
		for applicant in &self.data {
			println!("{}", &applicant);
		}
	}

}

impl Clone for HiringTable {
	fn clone(&self)->Self {
		HiringTable {
			data: self.data.clone()
		}	
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
		hiring_table.print_applicant_table();
		assert_eq!(hiring_table.data, vec![applicant_clone]);
	}

	#[test]
	fn size_of_hiring_table() {
		let mut hiring_table = HiringTable::new();
		let applicant = Applicant::from(
			vec!["facebook".to_string(), "meta".to_string()],
			"Tommy",
			3.2,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);
		hiring_table.add_applicant(applicant);
		println!("After adding");
		hiring_table.print_applicant_table();
		assert_eq!(hiring_table.size(), 1);
	}
	#[test]
	fn remove_applicant() {
		let mut hiring_table = HiringTable::new();
		let applicant = Applicant::from(
			vec!["facebook".to_string(), "meta".to_string()],
			"Tommy",
			3.2,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);
		hiring_table.add_applicant(applicant);
		println!("After adding");
		hiring_table.print_applicant_table();
		
		hiring_table.remove_applicant("Tommy");
		println!("After Removing: ");
		hiring_table.print_applicant_table();
		assert_eq!(hiring_table.size(), 0);
	}

	#[test]
	fn clone_hiring_table() {

		let mut hiring_table1 = HiringTable::new();
		let applicant = Applicant::from(
			vec!["facebook".to_string(), "meta".to_string()],
			"Tommy",
			3.2,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);
		hiring_table1.add_applicant(applicant);

		let mut hiring_table2 = hiring_table1.clone();
		hiring_table2.remove_applicant("Tommy");
		assert_eq!(hiring_table1.size(), 1);
		assert_eq!(hiring_table2.size(), 0);
	}


	#[test]
	fn filter_appliant_test() {
		
		let applicants = vec![
			Applicant::from (
				vec!["Company A".to_string()],
				"PersonA",
				3.7,
				"College X",
				vec!["Rust".to_string()],
			),
			
			Applicant::from (
				vec!["Company B".to_string()],
				"PersonB",
				3.5,
				"College Y",
				vec!["Python".to_string()],
			),
			Applicant::from (
				vec!["Company A".to_string()],
				"PersonC",
				3.9,
				"College X",
				vec!["Rust".to_string()],
			),

		];

		let table = HiringTable {
			data: applicants,
		};

		let filtered = HiringTable::filter_applicant(
			&table,
			Some("Company A".to_string()),
			Some("Rust".to_string()),
			None,
			None,
		);

		for applicant in &filtered {
			println!("{}", applicant);
		}
		assert_eq!(filtered.len(), 2)


	}

}
