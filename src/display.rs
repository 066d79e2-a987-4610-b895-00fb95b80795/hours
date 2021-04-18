use std::io::{stdout, Write};

use chrono::Duration;
use crossterm::{cursor, QueueableCommand};

use crate::util;

pub fn hide_cursor() {
    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();
    stdout.flush().unwrap();
}

pub fn show_cursor() {
    let mut stdout = stdout();
    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();
}

pub fn draw_duration(duration: Duration) {
    let duration = util::format_duration(duration);
    let mut stdout = stdout();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    write!(&mut stdout, "{}", duration).unwrap();
    stdout.flush().unwrap();
}
