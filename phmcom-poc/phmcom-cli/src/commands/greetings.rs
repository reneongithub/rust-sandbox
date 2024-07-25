use clap::{Arg, ArgMatches, Command};

use super::RegisterCommand;
pub struct GreetingCommand;

impl RegisterCommand for GreetingCommand {
    fn register(&self, command: Command) -> Command {
        command.subcommand(Command::new("greetings").arg(Arg::new("name").short('n').long("name")))
    }

    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<()> {
        if let Some(_matches) = matches.subcommand_matches("greetings") {
            println!(
                "let me greet you: {}",
                &_matches.get_one::<String>("name").unwrap()
            );
        }

        Ok(())
    }
}
