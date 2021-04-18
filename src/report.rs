use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::util;

pub struct Report(pub String);

impl Report {
    pub fn load() -> Self {
        match fs::read_to_string(report_path()) {
            Ok(s) => Self(s),
            Err(_) => Self("".to_owned()),
        }
    }

    pub fn save(&self) {
        fs::write(report_path(), &self.0).unwrap();
    }

    pub fn save_backup(&self) {
        fs::write(backup_report_path(), &self.0).unwrap();
    }

    pub fn commit_backup() {
        if Self::should_commit_backup() {
            println!(
                "Moving backup file \"{}\" to \"{}\".",
                backup_report_path().to_str().unwrap(),
                report_path().to_str().unwrap()
            );
            fs::rename(backup_report_path(), report_path()).unwrap();
        }
    }

    fn should_commit_backup() -> bool {
        let report_meta = fs::metadata(report_path());
        let backup_report_meta = fs::metadata(backup_report_path());
        if report_meta.is_err() && backup_report_meta.is_ok() {
            true
        } else if let (Ok(report_meta), Ok(backup_report_meta)) = (report_meta, backup_report_meta)
        {
            backup_report_meta.modified().unwrap() > report_meta.modified().unwrap()
        } else {
            false
        }
    }

    pub fn last_updated() -> Option<DateTime<Utc>> {
        fs::metadata(report_path())
            .ok()
            .map(|report_meta| report_meta.modified().unwrap())
            .map(|modified| DateTime::from(modified))
    }
}

fn report_path() -> PathBuf {
    util::home_dir().join("hours.txt")
}

fn backup_report_path() -> PathBuf {
    util::home_dir().join("hours.bak.txt")
}
