mod storage;
mod commands;

use std::error::Error;
use clap::{Parser, Subcommand};

use commands::building::build_command;

#[derive(Parser)]
pub struct Args {
	#[clap(subcommand)]
	branch: ActionBranch,
}

#[derive(Subcommand)]
pub enum ActionBranch {
	Env {
		#[clap(subcommand)]
		action: EnvAction, 
	},
	File {
		#[clap(subcommand)]
		action: FileAction,
	},
}

#[derive(Subcommand)]
pub enum EnvAction {
	Import {
		path: String, 
		name: String,
	},
	Export {
		path: String, 
		name: String,
	},
	Create {
		name: String,
	},
	Delete {
		name: String,
	},
	Use {
		name: String,
	},
	List {
		name: String,
	}
}

#[derive(Subcommand)]
pub enum FileAction {
	Add {
		path: String,
	},
	Remove {
		path: String,
	},
}

pub fn manage_secrets(args: Args) -> Result<(), Box<dyn Error>> {
	let mut command = build_command(args);
	command.execute()
}

