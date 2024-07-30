mod commands;

use env_logger::fmt::Formatter;
use log::Record;
use phmcom_config::config;
use std::io::Error as IoError;
use tokio::runtime::Builder;

const APP_PREFIX: &str = "PHMCOM";

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .format(log_format)
        .init();

    let config = config::Configuration::new("my/config/file.toml", APP_PREFIX);
    match config {
        Ok(c) => println!("configuration : {}", c.database.url),
        Err(e) => log::error!("{}", e),
    }

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    log::info!("Start application");
    log::debug!("lets check config crate : {}", config::say_hello_config());

    commands::handle_commands().await
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
