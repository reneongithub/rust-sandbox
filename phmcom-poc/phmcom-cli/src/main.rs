mod commands;

use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let command_config = commands::command_config();
    let mut cmd_root = commands::main_command();

    for command in &command_config {
        cmd_root = command.register(cmd_root);
    }

    let matches = cmd_root.get_matches();

    for command in &command_config {
        command.handle(&matches)?;
    }

    Ok(())
}
