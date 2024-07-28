use clap::{ArgMatches, Command};

use super::RegisterCommand;

const COMMAND_ID: &str = "hello";

pub struct HelloCommand;

impl RegisterCommand for HelloCommand {
    fn register(&self, command: Command) -> Command {
        command.subcommand(Command::new(COMMAND_ID))
    }

    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<bool> {
        println!("handle : {}", COMMAND_ID);
        Ok(matches
            .subcommand_matches(COMMAND_ID)
            .map_or(false, |_matches| {
                println!("Hello World!");
                true
            }))
    }
}
