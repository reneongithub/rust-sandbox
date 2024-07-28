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
    fn command_id(&self) -> String;
    fn config(&self, command: Command) -> Command;
    fn execute_handle(&self, matches: &ArgMatches) -> anyhow::Result<bool>;
    fn handle(&self, matches: &ArgMatches) -> anyhow::Result<bool> {
        log::debug!("handle command : {}", self.command_id());
        self.execute_handle(matches)
    }
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

#[macro_export]
macro_rules! impl_command_id {
    ($command:ty, $command_id:expr) => {
        impl $command {
            fn command_id(&self) -> String {
                $command_id.to_string()
            }
        }
    };
}
