use clap::Parser;

mod error;
mod run;
mod utils;

/// A BitTorrent client written in rust.
#[derive(Parser, Debug)]
#[command(author = "Mukund", version, about, long_about = None)]
struct Args {
    /// Path to the .torrent or .magnet file
    #[arg(short, long)]
    source: String,

    /// Directory to save the downloaded files too
    #[arg(short, long, default_value = ".")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match run::run(args).await {
        Ok(path) => {
            println!("Sucessfully obtained torrent file.");
            println!("Read to process torrent from: {}", path.display());
            todo!("Implement later")
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}
