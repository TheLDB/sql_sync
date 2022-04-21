use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ISqlSyncArgs { 
    // Args for clap CLI
    #[clap(long)]
    cN: Option<String>,
}

fn main() {
    let args = ISqlSyncArgs::parse();
    println!("{:?}", args);
}
