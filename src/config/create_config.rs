use serde::{Deserialize};
use std::fs::{File, copy};
use std::io::prelude::*;
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
    // Get Toml Config Template contents

    // Create new .toml config file for config (with template)
    File::create(format!("{}.toml", name))?;

    copy("./src/template/config.toml", format!("{}.toml", name))?;


    Ok(())
}