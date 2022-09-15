use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
	#[clap(subcommand)]
	branch: ActionBranch,
}

#[derive(Subcommand)]
enum ActionBranch {
	Set {
		#[clap(subcommand)]
		action: SetAction, 
	},
	File {
		#[clap(subcommand)]
		action: FileAction,
	},
}

#[derive(Subcommand)]
enum SetAction {
	Create,
	Delete,
	Use
}

#[derive(Subcommand)]
enum FileAction {
	Add,
	Remove,
}

pub fn manage(args: Args) {
	match args.branch {
		ActionBranch::Set { .. } => println!("Set commands"),
		ActionBranch::File { action } => {
			match action {
				FileAction::Add => println!("add"),
				_ => println!("other file action"),
			}
		}
		_ => println!("other")
	}
}

