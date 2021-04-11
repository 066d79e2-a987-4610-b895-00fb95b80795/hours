use std::io::{stdout, Write};

use chrono::Duration;
use crossterm::{cursor, QueueableCommand};

use crate::util;

pub fn draw_duration(duration: Duration) {
    let duration = util::format_duration(duration);
    let mut stdout = stdout();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    write!(&mut stdout, "{}", duration).unwrap();
    stdout.flush().unwrap();
}
