use clap::Parser;
use std::process::exit;
use mng_secrets::{Args, manage_secrets};

fn main() {
	let args = Args::parse();
	if let Err(error) = manage_secrets(args) {
		eprintln!("{error}");
		exit(1);
	}
}
