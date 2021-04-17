use std::{
    process,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{self, Instant},
};

use chrono::{Duration, Local};
use report::Report;
use timesheet::Timesheet;

mod display;
mod gist;
mod report;
mod timesheet;
mod util;

fn main() {
    Report::commit_backup();
    let start = Instant::now();
    let ctrlc_receiver = register_ctrlc_handler();
    let (handle, cancel_sender) = write_backups_in_background(start);
    while ctrlc_receiver.try_recv().is_err() {
        let duration = Duration::from_std(start.elapsed()).unwrap();
        display::draw_duration(duration);
    }
    cancel_sender.send(Cancel).unwrap();
    handle.join().unwrap();
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
        Report::commit_backup();
    });
    (handle, sender)
}
