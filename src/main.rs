// 1st Party Libs


// 3rd Party Libs
use clap::Parser;

// Local Files
pub mod config;
pub mod helpers;
use config::create_config::{create};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ISqlSyncArgs { 
    // Args for clap CLI

    /// Initialize sql_sync, create a new config w/ a name
    #[clap(short = 'i', long)]
    init: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ISqlSyncArgs::parse();

    if let Some(name) = args.init {
        create(name)?;
    }
    
    Ok(())
}
