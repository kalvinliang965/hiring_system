
use	hiring_system_simulation::HiringTable;
use hiring_system_simulation::Applicant;


#[test]
fn demo() {
	
	let mut table = HiringTable::new();

	let mark_zuck = Applicant::from_primative(
		&["Facebook"],
		"Mark Zuck",
		3.99,
		"Harvard",
		&["Business Management"]
	);

	let steve_jobs = Applicant::from_primative(
		&["Apple", "Google", "Two Sigma"],
		"Steve Jobs",
		3.98,
		"De Anza College",
		&["Ruby on Rails", "Java"],
	);
	let henry_white = Applicant::from_primative(
		&["AirBnb", "Facebook"],
		"Henry White",
		2.5,
		"NYIT",
		&["Javascript"],
	);
	
	table.add_applicant(mark_zuck.clone());
	table.add_applicant(steve_jobs.clone());
	table.add_applicant(henry_white.clone());
	
	// expect to see applicant added above
	table.print_applicant_table();
	println!();

	let found_applicant = table.get_applicant("Mark Zuck").expect("Mark Zuck not found");
	assert_eq!(found_applicant.get_applicant_name(), "Mark Zuck");
	assert_eq!(found_applicant.get_company_name(), &vec!["Facebook".to_string()]);
	assert_eq!(found_applicant.get_applicant_gpa(), 3.99);
	assert_eq!(found_applicant.get_applicant_college(), "Harvard");
	assert_eq!(found_applicant.get_applicant_skills(), &vec!["Business Management".to_string()]);
	assert_eq!(found_applicant, &mark_zuck);

	let bob_chen = Applicant::from_primative(
		&["Quora", "Google", "Twitter"],
		"Bob Chen",
		3.60,
		"Stony Brook",
		&["Java", "C++", "C"],
	);

	table.add_applicant(bob_chen.clone());

	assert_eq!(table.size(), 4);
	
	// expect to include bob
	table.print_applicant_table();
	println!();

	let filtered_applicants_one = HiringTable::filter_applicant(&table, Some("Facebook"), None, None, None);
	assert_eq!(filtered_applicants_one.len(), 2);
	assert_eq!(filtered_applicants_one[0], &mark_zuck);
	assert_eq!(filtered_applicants_one[1], &henry_white);
	println!("Filter applicant test one:");
	println!("{}", "_".repeat(40 + 20 + 6 + 20 + 40));
	for applicant in &filtered_applicants_one {
		println!("{}", applicant);
	}
	println!();
	

	let filtered_applicants_two = HiringTable::filter_applicant(&table,Some("Facebook"),None,  None, Some(3.0));
	assert_eq!(filtered_applicants_two.len(), 1);
	assert_eq!(filtered_applicants_two[0], &mark_zuck);
	println!("Filter applicant test two:");
	println!("{}", "_".repeat(40 + 20 + 6 + 20 + 40));
	for applicant in &filtered_applicants_two {
		println!("{}", applicant);
	}
	println!();

	let filtered_applicants_three = HiringTable::filter_applicant(&table, Some("Google"), Some("Java"), None, None);
	assert_eq!(filtered_applicants_three.len(), 2);
	assert_eq!(filtered_applicants_three[0], &steve_jobs);
	assert_eq!(filtered_applicants_three[1], &bob_chen);
	println!("Filter applicant test three:");
	println!("{}", "_".repeat(40 + 20 + 6 + 20 + 40));
	for applicant in &filtered_applicants_three {
		println!("{}", applicant);
	}
	println!();

	let mut backup = table.clone();
	assert_eq!(&backup, &table);
	let remove_applicant = table.remove_applicant("Mark Zuck").expect("Mark Zuck not found for removing");
	assert_eq!(remove_applicant, mark_zuck);
	assert_ne!(&backup, &table);
	table.print_applicant_table();
	println!();
}
