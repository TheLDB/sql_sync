use serde::{Serialize, Deserialize};

// Struct for Confy config
#[derive(Serialize, Deserialize, Debug)]
pub struct ISqlSyncConfig {
    pub config_name: String,
}

impl ::std::default::Default for ISqlSyncConfig {
    fn default() -> Self { Self { config_name: "".to_string() }}
}

pub fn create_config(name: &str) -> Result<ISqlSyncConfig, confy::ConfyError> {
    let sql_sync_config: ISqlSyncConfig = confy::load(name)?;
    Ok(sql_sync_config)
}