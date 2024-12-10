use std::fs::File;
use std::io::Write;
use std::io::{stdin, stdout, Error};
use std::fs;
use std::path::Path;

fn create_file(file_name: String, content: Vec<String>) -> Result<(), Error> {
	let mut f = File::create(file_name).expect("Error while creating file");
	for s in &content {
		f.write_all(format!("{}\n", s).as_bytes()).expect("Unable to write to file!");
	}
	Ok(())
}

fn main() {
	let mut project_name = String::new();
	let _ = stdout().flush();

	println!("Enter project name:");
	stdin().read_line(&mut project_name).expect("Not a valid strign!");
	
	project_name.pop();
	
	fs::create_dir(&project_name);
	
	create_file(
		Path::new(&project_name)
			.join("ansible.cfg")
			.into_os_string()
			.into_string()
			.expect("Cant create Path!"), vec!["[defaults]".to_string(), "host_key_checking = False".to_string(), "inventory = inventory".to_string()]
	).unwrap();

	create_file(
		Path::new(&project_name)
			.join("inventory")
			.into_os_string()
			.into_string()
			.expect("Cant create Path!"), vec!["[all]".to_string(), "host1 ansible_host= ansible_user=".to_string()]
	).unwrap();

	create_file(
		Path::new(&project_name)
			.join(format!("{}.yaml", &project_name))
			.into_os_string()
			.into_string()
			.expect("Cant create Path!"), vec![
				"---".to_string(), 
				"- name: Template Ansbile Playbook".to_string(),
				"  host: all".to_string(), 
				"  tasks:".to_string(),
				"    - name: Hello world".to_string(), 
				"      ansible.builtin.debug:".to_string(),
				"        msg: \"Hello World\"".to_string(),
				"".to_string()
			]
	).unwrap();
	
	println!("Project {} created!", project_name);
}
