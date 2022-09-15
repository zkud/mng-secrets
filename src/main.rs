use clap::Parser;
use mng_secrets::{Args, manage};

fn main() {
	let args = Args::parse();
	manage(args);
}
