use std::path::Path;

use crate::error::AppError;
use clap::Parser;
use lava_torrent::torrent::v1::Torrent;

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
            if let Err(e) = process_torrent(&path).await {
                eprintln!("Error processing torrent: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}

async fn process_torrent(torrent_path: &Path) -> Result<(), AppError> {
    let path_buf = torrent_path.to_path_buf();

    let torrent = tokio::task::spawn_blocking(move || Torrent::read_from_file(path_buf))
        .await
        .unwrap()?;

    println!("\n--- Torrent Metadata ---");

    if let Some(announce_url) = &torrent.announce {
        println!("Announce URL: {}", announce_url);
    } else {
        println!("Announce URL: Not specified");
    }

    if let Some(announce_list) = &torrent.announce_list {
        println!("Announce List:");
        for (i, tier) in announce_list.iter().enumerate() {
            println!("  Tier {}:", i + 1);
            for url in tier {
                println!("    - {}", url);
            }
        }
    }

    println!("Info Hash: {}", torrent.info_hash());
    println!("File/Directory Name: {}", torrent.name);
    println!("Piece Length: {} bytes", torrent.piece_length);

    let num_pieces = torrent.pieces.len() / 20;
    println!("Number of Pieces: {}", num_pieces);

    if let Some(files) = &torrent.files {
        let total_size: i64 = files.iter().map(|f| f.length).sum();
        println!("Mode: Multi-file ({} files)", files.len());
        println!("Total Size: {} bytes", total_size);
    } else {
        println!("Mode: Single-file");
        println!("Total Size: {} bytes", torrent.length);
    }

    println!("------------------------");

    Ok(())
}
