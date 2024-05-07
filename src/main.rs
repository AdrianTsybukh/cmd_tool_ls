use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use chrono::Datelike;
use chrono::SubsecRound;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    t.into()
}

fn format_item_date(datetime: DateTime<Utc>) -> String {
    let year = datetime.year();
    let month = datetime.month();
    let day = datetime.day();
    let time = datetime.time().trunc_subsecs(0);

    format!("{}-{}-{} {}", year, month, day, time)
}

fn read_directory(path: &str) -> Vec<PathBuf> {
    let entries = fs::read_dir(path).expect("[ERROR] could not read directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().expect("[ERROR] could not collect directory items");

    entries
}

fn format_entries(entries: Vec<PathBuf>) -> String {
    let mut output = String::new();

    for path in entries.iter() {
        let current_metadata = path.metadata().expect("[ERROR] could not get metadata");
        let created = system_time_to_date_time(current_metadata.created().expect("[ERROR] could not get creation time"));
        let modified = system_time_to_date_time(current_metadata.modified().expect("[ERROR] could not get last modification time"));
        let item_name = path.file_name().unwrap().to_str().unwrap();
        let file_size = if current_metadata.is_file() {
            current_metadata.file_size().to_string()
        } else {
            "<DIR>".to_string()
        };

        let current_info = format!("{}\t{}\t{}\t{}\n", format_item_date(created), format_item_date(modified), file_size, item_name);

        output.push_str(&current_info);
    }

    output
}

fn main() -> io::Result<()> {
    let entries = read_directory(".");
    let res = format_entries(entries);

    println!("{}", res);

    Ok(())
}
