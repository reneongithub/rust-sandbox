pub mod database;
pub mod server;

use database::DatabaseConfig;
use serde::Deserialize;
use server::ServerConfig;

#[derive(Debug, Deserialize, Default)]
pub struct Configuration {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl Configuration {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        log::debug!(
            "Build configuration with FILe: {} and PREFIX: {}",
            location,
            env_prefix
        );
        let database: DatabaseConfig = DatabaseConfig {
            url: "my-db-url".to_string(),
        };
        let server: ServerConfig = ServerConfig {
            port: "3306".to_string(),
        };

        Ok(Configuration {
            database: database,
            server: server,
        })
    }
}
