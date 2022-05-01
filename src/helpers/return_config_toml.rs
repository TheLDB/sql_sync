pub fn return_config_toml() -> Result<&'static str, Box<dyn std::error::Error>> {
    let config_toml = r#"# Default Config Template

name = "config"
    
[server_1]
ip = "192.168.1.1"
port = 3006
    
[server_2]
ip = "192.168.1.1"
port = 3006"#;

    Ok(config_toml)
}