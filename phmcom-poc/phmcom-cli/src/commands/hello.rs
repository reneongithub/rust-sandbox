use clap::{ArgMatches, Command};

// use super::RegisterCommand;

const COMMAND_ID: &str = "hello";

pub struct HelloCommand;

impl HelloCommand {
    pub fn command_id(&self) -> String {
        COMMAND_ID.to_string()
    }

    pub fn config(&self, command: Command) -> Command {
        command.subcommand(Command::new(COMMAND_ID))
    }

    pub fn execute_handle(&self, matches: &ArgMatches) -> anyhow::Result<bool> {
        Ok(matches
            .subcommand_matches(COMMAND_ID)
            .map_or(false, |_matches| {
                println!("Hello World!");
                true
            }))
    }
}
