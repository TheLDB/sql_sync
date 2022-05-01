use serde::Deserialize;
use std::fs::{copy, File};
use std::io::prelude::*;

use crate::helpers::return_config_toml::return_config_toml;

#[derive(Deserialize)]
struct Config {
    name: String,
    server_1: Server1,
    server_2: Server2,
}

#[derive(Deserialize)]
struct Server1 {
    ip: String,
    port: i16,
}

#[derive(Deserialize)]
struct Server2 {
    ip: String,
    port: i16,
}

pub fn create(name: String) -> Result<(), Box<dyn std::error::Error>> {
    // Get base toml config
    let config_toml = return_config_toml()?;

    // Create new .toml config file for config (with template)
    let mut config = File::create(format!("{}.toml", name))?;
    config.write_all(config_toml.as_bytes())?;
    

    Ok(())
}
