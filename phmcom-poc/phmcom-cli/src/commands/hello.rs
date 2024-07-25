use clap::{ArgMatches, Command};

use super::RegisterCommand;
pub struct HelloCommand;

impl RegisterCommand for HelloCommand {
    fn register(&self, command: Command) -> Command {
        command.subcommand(Command::new("hello"))
    }

    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<()> {
        if let Some(_matches) = matches.subcommand_matches("hello") {
            println!("Hello World!");
        }

        Ok(())
    }
}
