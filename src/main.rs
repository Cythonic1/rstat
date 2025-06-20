use std::{fs, path::PathBuf};
use clap::Parser;

/// Simple CLI to show file metadata
#[derive(Parser, Debug)]
struct Args {
    /// File name to inspect
    file_name: PathBuf,
}

fn print_data(file_name: &str) {
    match fs::metadata(file_name) {
        Ok(metadata) => {
            println!("File Metadata for '{}':", file_name);
            println!("  - File size: {} bytes", metadata.len());
            println!("  - Is directory: {}", metadata.is_dir());
            println!("  - Is file: {}", metadata.is_file());
            println!("  - Permissions: {:?}", metadata.permissions());
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                println!("  - Inode: {}", metadata.ino());
                println!("  - Mode: {:o}", metadata.mode());
            }
        }
        Err(e) => {
            eprintln!("Failed to get metadata for '{}': {}", file_name, e);
        }
    }
}

fn main() {
    let cli = Args::parse();

    println!("Thank you for the file: {:?}", cli.file_name);
    print_data(cli.file_name.to_str().unwrap());

    println!("Hello, world!");
}
