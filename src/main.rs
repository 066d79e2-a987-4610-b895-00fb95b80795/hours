use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::{self, Instant},
};

use chrono::{DateTime, Duration, FixedOffset, Local};
use gist::GistClient;
use remaining_work::IncludeToday;
use report::Report;
use settings::Settings;
use timesheet::Timesheet;

mod gist;
mod remaining_work;
mod report;
mod settings;
mod terminal;
mod timesheet;
mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Report::commit_backup();
    sync_gist().await;

    let start = Instant::now();
    let duration_for_today = load_timesheet().get_hours(&Local::today());
    let (handle, backup_cancel_sender) = write_backups_in_background(start);
    terminal::init();
    while !terminal::ctrlc() {
        let duration = Duration::from_std(start.elapsed()).unwrap() + duration_for_today;
        terminal::draw_duration(duration);
    }
    terminal::quit();
    println!();
    backup_cancel_sender.send(Cancel).unwrap();
    handle.join().unwrap();

    Report::commit_backup();
    sync_gist().await;
    show_remaining_work();
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

fn show_remaining_work() {
    let timesheet = load_timesheet();
    if let Some(work) = timesheet.remaining_work() {
        println!(
            "Including today, {} days remaining this month, which is about {} work per day.",
            work.num_working_days(IncludeToday::Yes),
            util::format_duration(work.time_per_day(IncludeToday::Yes))
        );
        println!(
            "Not including today, {} days remaining this month, which is about {} work per day.",
            work.num_working_days(IncludeToday::No),
            util::format_duration(work.time_per_day(IncludeToday::No))
        );
    }
}
