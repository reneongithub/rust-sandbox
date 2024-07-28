mod commands;

use env_logger::fmt::Formatter;
use log::Record;
use std::io::Error as IoError;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .format(log_format)
        .init();

    log::info!("Application started");

    let command_config = commands::command_config();
    let mut cmd_root = commands::main_command();

    for command in &command_config {
        cmd_root = command.config(cmd_root);
    }

    let matches = cmd_root.get_matches();

    for command in &command_config {
        match command.handle(&matches) {
            Ok(false) => continue,
            Ok(true) => return Ok(()),
            Err(e) => {
                log::error!("Executing command failed : {}", e);
                return Err(e);
            }
        }
    }

    log::warn!("No command found for execute!");

    Ok(())
}

fn log_format(f: &mut Formatter, record: &Record<'_>) -> Result<(), IoError> {
    use std::io::Write as _;

    let ts = f.timestamp_nanos();
    let level = f.default_level_style(record.level());
    let module = record.module_path();
    let file = record.file();
    let line = record.line();
    let args = record.args();

    write!(f, "{level:<5} {ts}")?;
    if let Some(module) = module {
        write!(f, " {module}")?;
    }
    if let Some(file) = file {
        write!(f, " ({file}")?;
        if let Some(line) = line {
            write!(f, ":{line}")?;
        }
        write!(f, ")")?;
    }
    writeln!(f, "\n{args}\n")?;

    Ok(())
}
