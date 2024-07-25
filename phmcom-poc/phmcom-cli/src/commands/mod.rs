pub mod greetings;
pub mod hello;

use clap::{ArgMatches, Command};
use greetings::GreetingCommand;
use hello::HelloCommand;

pub trait RegisterCommand {
    fn register(&self, command: Command) -> Command;
    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<()>;
}

pub fn command_registry() -> Vec<Box<dyn RegisterCommand>> {
    vec![Box::new(HelloCommand), Box::new(GreetingCommand)]
}
