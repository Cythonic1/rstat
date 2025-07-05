use std::{
    fs::{self, Metadata},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::SystemTime,
};
use clap::Parser;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use tabled::{settings::Style, Tabled};
use chrono::{DateTime, Local, TimeZone, Utc};
use tree_magic_mini::{self, from_filepath};
use users::{get_user_by_uid, get_group_by_gid};

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

#[derive(Debug, Tabled, Default, Serialize, Deserialize)]
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
    uid: String,
    gid: String,
    sympolic: bool,
    access_date: String,
    modify_date: String,
    change_data: String,
    birth: String,
    file_type: &'static str,
}

impl std::fmt::Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File Name: {}", self.name)?;
        writeln!(f, "Size: {} bytes", self.size)?;
        writeln!(f, "Blocks: {}", self.blocks)?;
        writeln!(f, "IO Block Size: {}", self.io_block)?;
        writeln!(f, "Devices: {}", self.devices)?;
        writeln!(f, "Entry Type: {:?}", self.entry_type)?;
        writeln!(f, "Inode: {}", self.inod)?;
        writeln!(f, "Links: {}", self.links)?;
        writeln!(f, "Access Permissions: {}", self.access)?;
        writeln!(f, "UID: {}", self.uid)?;
        writeln!(f, "GID: {}", self.gid)?;
        writeln!(f, "Symbolic Link: {}", self.sympolic)?;
        writeln!(f, "Access Date: {}", self.access_date)?;
        writeln!(f, "Modify Date: {}", self.modify_date)?;
        writeln!(f, "Change Date: {}", self.change_data)?;
        writeln!(f, "Birth Date: {}", self.birth)?;
        writeln!(f, "Detected File Type: {}", self.file_type)
    }
}

#[derive(Debug, strum::Display, Default, Serialize, Deserialize)]
enum Type {
    Dir,
    #[default]
    File,
}

fn from_unix_to_normal_time(time: i64)  -> String{

    // Convert to DateTime<Utc>
    let dt: DateTime<Utc> = Utc.timestamp_opt(time, 0).unwrap();

    // Format the DateTime<Utc> to a readable string
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
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
    let file_path = Path::new(file_name);

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
        uid: format!("{:?}",get_user_by_uid(metadata.uid()).unwrap()),
        gid: format!("{:?}", get_group_by_gid(metadata.gid()).unwrap()),
        sympolic: metadata.file_type().is_symlink(),
        access_date: format_time(metadata.accessed()),
        modify_date: format_time(metadata.modified()),
        change_data: from_unix_to_normal_time(metadata.ctime()),
        birth: format_time(metadata.created()),
        file_type: from_filepath(file_path).unwrap()
    }
}

fn print_data_table(file_name: &str) {
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

fn print_data_normal(file_name: &str) {
    match fs::metadata(file_name) {
        Ok(metadata) => {
            let data = extract_data(metadata, file_name);
            println!("{}",data);
        }
        Err(e) => {
            eprintln!("Failed to get metadata for '{}': {}", file_name, e);
        }
    }
}

fn print_data_json(file_name: &str)  {
    match fs::metadata(file_name) {
        Ok(metadata) => {
            let data = extract_data(metadata, file_name);
            let json = serde_json::to_string_pretty(&data).unwrap();
            println!("{:#}",json);
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
        PrintType::Table => {
            print_data_table(cli.file_name.to_str().unwrap_or("."));
        },
        PrintType::Json => {
            print_data_json(cli.file_name.to_str().unwrap_or("."));
        }
        _ => {
            print_data_normal(cli.file_name.to_str().unwrap_or("."));
        }
    }
}
