use clap::{Arg, Command};
use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let command = Command::new("Dog Shelter sample application")
        .version("1.0")
        .author("Rene Kuehnemann <rene@wmedia.de>")
        .about("Say hello poc")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    _ = command.get_matches();

    Ok(())
}
