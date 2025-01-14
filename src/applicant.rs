
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

	pub fn from(company_name: Vec<String>, applicant_name: String,
			applicant_gpa: f64, applicant_college: String
			,applicant_skills: Vec<String>) -> Self {

		Applicant {
			company_name,
			applicant_name,
			applicant_gpa,
			applicant_college,
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
			String::from("Tommy"),
			3.69,
			String::from("Stony Brook"),
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


}
