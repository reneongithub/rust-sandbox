use clap::{Arg, ArgMatches, Command};

use super::RegisterCommand;

const COMMAND_ID: &str = "greetings";

pub struct GreetingCommand;

impl RegisterCommand for GreetingCommand {
    fn register(&self, command: Command) -> Command {
        command.subcommand(Command::new(COMMAND_ID).arg(Arg::new("name").short('n').long("name")))
    }

    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<bool> {
        Ok(matches
            .subcommand_matches(COMMAND_ID)
            .map_or(false, |_matches| {
                println!(
                    "let me greet you: {}",
                    &_matches.get_one::<String>("name").unwrap()
                );
                true
            }))
    }
}
