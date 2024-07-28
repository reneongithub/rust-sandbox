mod greetings;
mod hello;

use clap::{ArgMatches, Command};
use greetings::GreetingCommand;
use hello::HelloCommand;

const MAIN_COMMAND_ID: &str = "phmcom";
const MAIN_COMMAND_VERSION: &str = "0.1.0";
const MAIN_COMMAND_ABOUT: &str = "Lets work on a poc of phmcom in rust";
const MAIN_COMMAND_AUTHOR: &str = "Rene Kuehnemann <rene@wmedia.de>";

pub trait RegisterCommand {
    fn register(&self, command: Command) -> Command;
    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<bool>;
}

pub fn main_command() -> Command {
    Command::new(MAIN_COMMAND_ID)
        .version(MAIN_COMMAND_VERSION)
        .author(MAIN_COMMAND_AUTHOR)
        .about(MAIN_COMMAND_ABOUT)
}

pub fn command_config() -> Vec<Box<dyn RegisterCommand>> {
    vec![Box::new(HelloCommand), Box::new(GreetingCommand)]
}
