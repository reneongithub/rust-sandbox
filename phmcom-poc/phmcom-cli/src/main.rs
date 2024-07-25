mod commands;

use clap::Command;
use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let command_registry: Vec<Box<dyn commands::RegisterCommand>> = commands::command_registry();

    let mut cmd_root = Command::new("phmcom")
        .version("1.0")
        .author("Rene Kuehnemann <rene@wmedia.de>")
        .about("Lets work on a poc of phmcom in rust");

    for command in &command_registry {
        cmd_root = command.register(cmd_root);
    }

    let matches = cmd_root.get_matches();

    for command in &command_registry {
        command.handle(&matches)?;
    }

    Ok(())
}
