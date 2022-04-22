// 1st Party Libs


// 3rd Party Libs
use clap::Parser;

// Local Files
pub mod config;
use config::create_config::{create_config};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ISqlSyncArgs { 
    // Args for clap CLI

    /// Create New Config - Config Name
    #[clap(short = 'c', long)]
    create: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ISqlSyncArgs::parse();

    if let Some(create) = args.create {
        // Create new config file
        let test = create_config(&create)?;
        println!("{:?}", test);
    }
    Ok(())
}
