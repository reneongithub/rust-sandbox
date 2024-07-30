#[derive(Debug, serde::Deserialize, Default)]
pub struct DatabaseConfig {
    pub url: String,
}
