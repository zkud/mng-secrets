use std::error::Error;

pub trait Command {
	fn execute(&mut self) -> Result<(), Box<dyn Error>>;
}
