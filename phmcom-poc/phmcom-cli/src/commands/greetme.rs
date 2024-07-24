use clap::{Arg, ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command.subcommand(Command::new("greetme").arg(Arg::new("name").short('n').long("name")))
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("greetme") {
        println!(
            "let me greet you: {}",
            &_matches.get_one::<String>("name").unwrap()
        );
    }

    Ok(())
}
