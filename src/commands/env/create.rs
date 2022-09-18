use super::super::traits::Command;
use std::error::Error;

pub struct CreateEnvCommand {
	name: String,
}

impl CreateEnvCommand {
	pub fn new(name: String) -> Self {
		CreateEnvCommand { name }
	}
}

impl Command for CreateEnvCommand {
	fn execute(&mut self) -> Result<(), Box<dyn Error>> {
		println!("Create env with name {}", self.name);
		Ok(())
	}
}

