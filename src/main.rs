use std::{
    fs::{self, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::SystemTime,
};
use clap::Parser;
use clap::Subcommand;
use tabled::{settings::Style, Tabled};
use chrono::{DateTime, Local};

/// Simple CLI to show file metadata
#[derive(Parser, Debug)]
struct Args {
    /// File name to inspect
    file_name: PathBuf,
    #[clap(subcommand)]
    style: Option<PrintType>
}

#[derive(Debug, Subcommand)]
enum PrintType {
    /// Output in JSON format
    Json,
    /// Output in table format
    Table,
    /// Output in human-readable text (default)
    Normal,
}

#[derive(Debug, Tabled, Default)]
struct FileData {
    name: String,
    size: u64,
    blocks: u64,
    io_block: u64,
    devices: u64,
    entry_type: Type,
    inod: u64,
    links: u64,
    access: String,
    uid: u32,
    gid: u32,
    sympolic: bool,
    access_date: String,
    modify_date: String,
    change_data: String,
    birth: String,
}

#[derive(Debug, strum::Display, Default)]
enum Type {
    Dir,
    #[default]
    File,
}

fn format_time(time: Result<SystemTime, std::io::Error>) -> String {
    match time {
        Ok(t) => {
            let datetime: DateTime<Local> = t.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "Unavailable".to_string(),
    }
}

fn extract_data(metadata: Metadata, file_name: &str) -> FileData {
    let entry_type = if metadata.is_dir() {
        Type::Dir
    } else {
        Type::File
    };

    FileData {
        name: file_name.to_string(), 
        size: metadata.len(),
        blocks: metadata.blocks(),
        io_block: metadata.blksize(),
        devices: metadata.dev(),
        entry_type,
        inod: metadata.ino(),
        links: metadata.nlink(),
        access: format!("{:?}", metadata.permissions()),
        uid: metadata.uid(),
        gid: metadata.gid(),
        sympolic: metadata.file_type().is_symlink(),
        access_date: format_time(metadata.accessed()),
        modify_date: format_time(metadata.modified()),
        change_data: metadata.ctime().to_string(),
        birth: format_time(metadata.created()),
    }
}

fn print_data(file_name: &str) {
    match fs::metadata(file_name) {
        Ok(metadata) => {
            let data = extract_data(metadata, file_name);
            let mut table = tabled::Table::new(vec![data]);
            table.with(Style::rounded());
            println!("{}",table);
        }
        Err(e) => {
            eprintln!("Failed to get metadata for '{}': {}", file_name, e);
        }
    }
}

fn main() {
    let cli = Args::parse();
    let style = cli.style.unwrap_or(PrintType::Normal);


    match style {
        PrintType::Normal => {
            print_data(cli.file_name.to_str().unwrap_or("."));
        },
        _ => {
            todo!()
        }
    }
}
