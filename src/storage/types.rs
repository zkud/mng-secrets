use std::error::Error;

pub trait Storage {
	fn add_file(path: String) -> Result<(), Box<dyn Error>>;

	fn remove_file(path: String) -> Result<(), Box<dyn Error>>;
}
