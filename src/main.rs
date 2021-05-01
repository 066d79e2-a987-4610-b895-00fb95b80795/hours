use std::{
    process,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{self, Instant},
};

use chrono::{DateTime, Duration, FixedOffset, Local};
use gist::GistClient;
use report::Report;
use settings::Settings;
use timesheet::Timesheet;

mod display;
mod gist;
mod report;
mod settings;
mod timesheet;
mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Report::commit_backup();
    sync_gist().await;

    let start = Instant::now();
    let duration_for_today = load_timesheet().get_hours(&Local::today());
    let ctrlc_receiver = register_ctrlc_handler();
    let (handle, backup_cancel_sender) = write_backups_in_background(start);
    display::hide_cursor();
    while ctrlc_receiver.try_recv().is_err() {
        let duration = Duration::from_std(start.elapsed()).unwrap() + duration_for_today;
        display::draw_duration(duration);
    }
    display::show_cursor();
    println!();
    backup_cancel_sender.send(Cancel).unwrap();
    handle.join().unwrap();

    Report::commit_backup();
    sync_gist().await;
}

async fn sync_gist() {
    let report = Report::load();
    let settings = Settings::load();
    let gist_client = GistClient::new(settings.api_key.clone(), settings.gist_id.clone());
    let res = gist_client.get().await;
    if report.0.trim() != res.report.0.trim() {
        if should_update_local_file_from_gist(res.last_updated) {
            println!(
                "Updating local file from gist. New content:\n{}",
                res.report.0.trim()
            );
            res.report.save();
        } else {
            println!(
                "Updating gist from local file. New content:\n{}",
                report.0.trim()
            );
            gist_client.update(&report).await;
        }
    }
}

fn should_update_local_file_from_gist(gist_last_updated: DateTime<FixedOffset>) -> bool {
    match Report::last_updated() {
        None => true, // Local file does not exist
        Some(last_updated) if last_updated < gist_last_updated => true,
        _ => false,
    }
}

fn register_ctrlc_handler() -> Receiver<()> {
    let mut quit = false;
    let (sender, receiver) = mpsc::channel();
    ctrlc::set_handler(move || {
        if quit {
            process::exit(1);
        }

        sender.send(()).unwrap();
        quit = true;
    })
    .unwrap();
    receiver
}

struct Cancel;

fn write_backups_in_background(start: Instant) -> (thread::JoinHandle<()>, Sender<Cancel>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let today = Local::today();
        while receiver.try_recv().is_err() {
            thread::sleep(time::Duration::from_secs(3));
            let mut timesheet = load_timesheet();
            timesheet.add_hours(&today, &Duration::from_std(start.elapsed()).unwrap());
            timesheet.generate_report().save_backup();
        }
    });
    (handle, sender)
}

fn load_timesheet() -> Timesheet {
    let report = Report::load();
    Timesheet::parse_report(&report)
}
