use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command.subcommand(Command::new("hello"))
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("hello") {
        println!("Hello World!");
    }

    Ok(())
}
