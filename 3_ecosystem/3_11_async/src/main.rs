use std::path::PathBuf;
use clap::Parser;

/// util to download web pages
#[derive(Parser)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// maximum thread usage
    max_threads: Option<u16>
    
    file: PathBuf
}

#[tokio::main]
async fn main() {
    let args = Args::parse()



    println!("Implement me!");
}
