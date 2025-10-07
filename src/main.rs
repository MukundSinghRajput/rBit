use clap::Parser;

mod error;

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

    println!("Torrent file {}", args.source);
    println!("Output directory: {}", args.output)
}
