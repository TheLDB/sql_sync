use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ISqlSyncArgs { 
    // Args for clap CLI

    /// Create New Config - Config Name
    #[clap(short = 'c', long = "cn")]
    cn: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ISqlSyncArgs::parse();
    if let Some(cn) = args.cn {
        let mut new_config = File::create(format!("{}.toml", cn))?;
    }

    Ok(())
}
