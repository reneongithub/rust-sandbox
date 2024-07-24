use clap::Command;
use dotenv::dotenv;
use phmcom_cli::commands;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut cmd = Command::new("phmcom")
        .version("1.0")
        .author("Rene Kuehnemann <rene@wmedia.de>")
        .about("Lets work on a poc of phmcom in rust");

    cmd = commands::configure_hello(cmd);
    cmd = commands::configure_greetme(cmd);

    let matches = cmd.get_matches();

    commands::handle_hello(&matches)?;
    commands::handle_greetmee(&matches)?;

    Ok(())
}
