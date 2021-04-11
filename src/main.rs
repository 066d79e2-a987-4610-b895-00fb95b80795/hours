use std::{
    process,
    sync::mpsc::{self, Receiver},
    thread,
    time::Instant,
};

use chrono::Duration;

mod display;
mod gist;
mod timesheet;
mod util;

fn main() {
    let ctrlc_receiver = register_ctrlc_handler();
    let start = Instant::now();
    while ctrlc_receiver.try_recv().is_err() {
        let duration = Duration::from_std(start.elapsed()).unwrap();
        display::draw_duration(duration);
    }
    println!("\nSaving the timesheet. Ctrl+C to force-quit.");
    thread::sleep(std::time::Duration::from_secs(2));
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
