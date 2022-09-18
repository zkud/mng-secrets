use super::super::{Args, ActionBranch, FileAction, EnvAction};
use super::traits::Command;

use super::env::create::CreateEnvCommand;

pub fn build_command(args: Args) -> Box<dyn Command> {
	match args.branch {
		ActionBranch::Env { action } => {
			match action {
				EnvAction::Create { name } => Box::new(CreateEnvCommand::new(name)),
				_ => panic!("Unsupported for now")
			}
		},
		_ => panic!("Unsupported for now")
	}
}
