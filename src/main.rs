use std::{
    process,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{self, Instant},
};

use chrono::{Duration, Local};
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
    let ctrlc_receiver = register_ctrlc_handler();
    let (handle, backup_cancel_sender) = write_backups_in_background(start);
    while ctrlc_receiver.try_recv().is_err() {
        let duration = Duration::from_std(start.elapsed()).unwrap();
        display::draw_duration(duration);
    }
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
        if let Some(last_updated) = Report::last_updated() {
            if last_updated < res.last_updated {
                println!(
                    "Updating local file from gist. New content: {}.",
                    res.report.0
                );
                res.report.save();
            } else {
                println!("Updating gist from local file. New content: {}.", report.0);
                gist_client.update(&report).await;
            }
        }
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
            let report = Report::load();
            let mut timesheet = Timesheet::parse_report(&report);
            timesheet.add_hours(&today, &Duration::from_std(start.elapsed()).unwrap());
            timesheet.generate_report().save_backup();
        }
    });
    (handle, sender)
}
