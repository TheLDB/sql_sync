use serde::{Serialize, Deserialize};

use crate::ISqlSyncArgs;

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

pub fn update_config(name: &str, conf: &ISqlSyncConfig) -> Result<ISqlSyncConfig, confy::ConfyError> {
    let update_config = confy::store(name, conf)?;

    let updated_conf: ISqlSyncConfig = confy::load(name)?;

    Ok(updated_conf)
}