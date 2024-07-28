mod greetings;
mod hello;

use clap::Command;
use greetings::GreetingCommand;
use hello::HelloCommand;

const MAIN_COMMAND_ID: &str = "phmcom";
const MAIN_COMMAND_VERSION: &str = "0.1.0";
const MAIN_COMMAND_ABOUT: &str = "Lets work on a poc of phmcom in rust";
const MAIN_COMMAND_AUTHOR: &str = "Rene Kuehnemann <rene@wmedia.de>";

macro_rules! handle_command {
    ($cmd:expr, $matches:expr) => {
        match $cmd.execute_handle($matches) {
            Ok(true) => return Ok(()),
            Ok(false) => log::debug!("Command not handled : {}", $cmd.command_id()),
            Err(e) => {
                log::error!("Executing command failed: {}", e);
                return Err(e);
            }
        }
    };
}

pub fn command_configuration() -> anyhow::Result<()> {
    let mut cmd_root = main_command();

    cmd_root = HelloCommand.config(cmd_root);
    cmd_root = GreetingCommand.config(cmd_root);

    let matches = cmd_root.get_matches();

    handle_command!(HelloCommand, &matches);
    handle_command!(GreetingCommand, &matches);

    Ok(())
}

pub fn main_command() -> Command {
    Command::new(MAIN_COMMAND_ID)
        .version(MAIN_COMMAND_VERSION)
        .author(MAIN_COMMAND_AUTHOR)
        .about(MAIN_COMMAND_ABOUT)
}
