use std::fmt;
#[derive(Debug)]
pub struct Applicant {
	company_name: Vec<String>,
	applicant_name: String,
	applicant_gpa: f64,
	applicant_college: String,
	applicant_skills: Vec<String>
}

impl Applicant {
	
	pub fn new() -> Self {
		Applicant {
			company_name: Vec::new(),
			applicant_name: String::new(),
			applicant_gpa: 0.0,
			applicant_college: String::new(),
			applicant_skills: Vec::new(),
		}
	}

	pub fn from(company_name: Vec<String>, applicant_name: &str,
			applicant_gpa: f64, applicant_college: &str
			,applicant_skills: Vec<String>) -> Self {

		Applicant {
			company_name,
			applicant_name: String::from(applicant_name),
			applicant_gpa,
			applicant_college: String::from(applicant_college),
			applicant_skills,
		}
	}

	// getters
	pub fn get_company_name(&self) -> &Vec<String> {
		&self.company_name
	}

	pub fn get_applicant_name(&self) -> &str {
		&self.applicant_name
	}
	
	pub fn get_applicant_gpa(&self) -> f64 {
		self.applicant_gpa
	}

	pub fn get_applicant_college(&self) -> &str {
		&self.applicant_college
	}
	
	pub fn get_applicant_skills(&self) -> &Vec<String> {
		&self.applicant_skills
	}

	
	// setters
	pub fn set_company_name(&mut self, company_name: Vec<String>) {
		self.company_name = company_name;
	}
	
	pub fn set_applicant_name(&mut self, applicant_name: &str) {
		self.applicant_name = String::from(applicant_name);
	}

	pub fn set_applicant_gpa(&mut self, applicant_gpa: f64) {
		self.applicant_gpa = applicant_gpa;
	}

	pub fn set_applicant_college(&mut self, applicant_college: &str) {
		self.applicant_college = String::from(applicant_college);
	}

	pub fn set_applicant_skills(&mut self, applicant_skills: Vec<String>) {
		self.applicant_skills = applicant_skills;
	}
}


impl Clone for Applicant {
	fn clone(&self) -> Self {
		Applicant::from(
			self.get_company_name().clone(),
			self.get_applicant_name(),
			self.get_applicant_gpa(),
			self.get_applicant_college(),
			self.get_applicant_skills().clone(),
		)
	}
}

impl PartialEq for Applicant {
	fn eq(&self, other: &Self) -> bool {
		self.get_company_name() == other.get_company_name() 
		&& self.get_applicant_name() == other.get_applicant_name()
		&& self.get_applicant_gpa() == other.get_applicant_gpa()
		&& self.get_applicant_college() == other.get_applicant_college()
		&& self.get_applicant_skills() == other.get_applicant_skills()
	}
}

impl fmt::Display for Applicant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		
		let company_name = self.get_company_name()
			.iter()
			.map(|company| company.as_str())
			.collect::<Vec<&str>>()
			.join(", ");

		let applicant_name: &str = self.get_applicant_name();
		let applicant_gpa: f64 = self.get_applicant_gpa();
		let applicant_college: &str = self.get_applicant_college();

		let applicant_skills= self.get_applicant_skills()
			.iter()
			.map(|skill| skill.as_str())
			.collect::<Vec<&str>>()
			.join(", ");

		write!(f, "{:<40} {:<20} {:<6} {:<20} {:<40}",
			company_name,
			applicant_name,
			applicant_gpa,
			applicant_college,
			applicant_skills)
	}
}

#[cfg(test)]
mod tests {
	
	use super::*;
	
	// testing constructor 

	#[test]
	fn construct_members_new() {
		let applicant = Applicant::new();
		
		assert!(applicant.company_name.is_empty());
		assert!(applicant.applicant_name.is_empty());
		assert!(applicant.applicant_gpa == 0.0);
		assert!(applicant.applicant_college.is_empty());
		assert!(applicant.applicant_skills.is_empty());
	}
	
	#[test]
	fn construct_members_from() {
		
		let applicant = setup();

		// assert each field
		assert_eq!(
			applicant.company_name, vec!["Google", "Amazon", "Facebook"]
		);
		assert_eq!(
			applicant.applicant_name, String::from("Tommy")
		);
		assert_eq!(applicant.applicant_gpa, 3.69);
		assert_eq!(applicant.applicant_college, String::from("Stony Brook"));
		assert_eq!(applicant.applicant_skills, vec!["java", "python"]);
	}
	
	
	fn setup() -> Applicant {
		Applicant::from(
			vec!["Google".to_string(), "Amazon".to_string(), "Facebook".to_string()],
			"Tommy",
			3.69,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		)
	}
	// testing getters

	#[test]
	fn get_company_name() {
		let applicant = setup();
		assert_eq!(
			applicant.get_company_name(),
			&vec!["Google", "Amazon", "Facebook"]
		);
	}

	#[test]
	fn get_applicant_name() {
		let applicant = setup();
		assert_eq!(
			applicant.get_applicant_name(),
			"Tommy"
		);
	}

	#[test]
	fn get_applicant_gpa() {
		let applicant = setup();
		assert_eq!(
			applicant.get_applicant_gpa(),
			3.69
		);
	}

	#[test]
	fn get_applicant_college() {
		let applicant = setup();
		assert_eq!(
			applicant.get_applicant_college(),
			"Stony Brook"
		);
	}
	
	#[test]
	fn get_applicant_skills() {
		let applicant = setup();
		assert_eq!(
			applicant.get_applicant_skills(),
			&vec!["java", "python"]
		);
	}

	#[test]
	fn set_company_name() {
		let mut applicant = Applicant::new();
		applicant.set_company_name(vec!["CICA".to_string(), "X".to_string(), "HYALON".to_string()]);
		assert_eq!(applicant.get_company_name(), &vec!["CICA", "X", "HYALON"]);
		
		applicant.set_company_name(vec!["CICA".to_string(), "X".to_string()]);
		assert_eq!(applicant.get_company_name(), &vec!["CICA", "X"]);
	}

	#[test]
	fn set_applicant_name() {
		let mut applicant = Applicant::new();
		
		const original_name: &str = "original name";
		const modified_name: &str = "modified name";
		
		applicant.set_applicant_name(original_name);
		assert_eq!(applicant.get_applicant_name(), original_name);

		applicant.set_applicant_name(modified_name);
		assert_eq!(applicant.get_applicant_name(), modified_name);
	}

	#[test]
	fn set_applicant_gpa() {
		let mut applicant = Applicant::new();

		const original_gpa: f64 = 4.0;
		const modified_gpa: f64 = 3.0;

		applicant.set_applicant_gpa(original_gpa);
		assert_eq!(applicant.get_applicant_gpa(), original_gpa);

		applicant.set_applicant_gpa(modified_gpa);
		assert_eq!(applicant.get_applicant_gpa(), modified_gpa);
	}
	
	#[test]
	fn set_applicant_college() {
		let mut applicant = Applicant::new();

		const original_college: &str = "Harvard";
		const current_college: &str = "BMCC";

		applicant.set_applicant_college(original_college);
		assert_eq!(applicant.get_applicant_college(), original_college);

		applicant.set_applicant_college(current_college);
		assert_eq!(applicant.get_applicant_college(), current_college);
	}
	
	#[test]
	fn set_applicant_skills() {
		let mut applicant = Applicant::new();

		let original_skills: Vec<String> = vec!["C", "Java", "Python"]
											.into_iter()
											.map(String::from)
											.collect::<Vec<String>>();
		let current_skills: Vec<String> = vec!["C", "Java", "Python", "Rust"]
											.into_iter()
											.map(String::from)
											.collect::<Vec<String>>();
		
		applicant.set_applicant_skills(original_skills.clone());
		assert_eq!(applicant.get_applicant_skills(), &original_skills);

	 	applicant.set_applicant_skills(current_skills.clone());
		assert_eq!(applicant.get_applicant_skills(), &current_skills);
	}


	#[test]
	fn compare_after_clone() {
		let applicant1 = setup();
		let applicant2 = applicant1.clone();
		assert_eq!(applicant1, applicant2);
	}

	#[test]
	fn compare_equal() {
		let applicant1 = setup();
		let applicant2 = Applicant::from(
			vec!["Google".to_string(), "Amazon".to_string(), "Facebook".to_string()],
			"Tommy",
			3.69,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);
		assert_eq!(applicant1, applicant2);
	}

	#[test]
	fn compare_not_equal() {
		let applicant1 = setup();
		let applicant2 = Applicant::from(
			vec!["Google".to_string(), "amz".to_string(), "Facebook".to_string()],
			"Tommy",
			3.69,
			"Stony Brook",
			vec!["java".to_string(), "python".to_string()]
		);
		assert_ne!(applicant1, applicant2);
	}

	#[test]
	fn test_clone_independecy() {
		let applicant1 = setup();
		let mut applicant2 = applicant1.clone();
		// modify applicant2
		applicant2.set_applicant_gpa(2.0);
		assert_ne!(applicant1, applicant2);
	}

}
