use std::fs;
use std::{env, path::PathBuf};

pub struct Report(pub String);

impl Report {
    pub fn load() -> Self {
        match fs::read_to_string(report_path()) {
            Ok(s) => Self(s),
            Err(_) => Self("".to_owned()),
        }
    }

    pub fn save_backup(&self) {
        fs::write(backup_report_path(), &self.0).unwrap();
    }

    pub fn commit_backup() {
        let report_meta = fs::metadata(report_path());
        let backup_report_meta = fs::metadata(backup_report_path());
        if let (Ok(report_meta), Ok(backup_report_meta)) = (report_meta, backup_report_meta) {
            if backup_report_meta.modified().unwrap() > report_meta.modified().unwrap() {
                fs::rename(backup_report_path(), report_path()).unwrap();
            }
        }
    }
}

fn home_dir() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap())
}

fn report_path() -> PathBuf {
    home_dir().join("hours.txt")
}

fn backup_report_path() -> PathBuf {
    home_dir().join("hours.bak.txt")
}
