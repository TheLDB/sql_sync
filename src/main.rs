// 1st Party Libs


// 3rd Party Libs
use clap::Parser;

// Local Files
pub mod config;

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
        // Create new config file
        let test = config::create_config::create_config(&cn)?;
        println!("{:?}", test);
    }

    Ok(())
}
