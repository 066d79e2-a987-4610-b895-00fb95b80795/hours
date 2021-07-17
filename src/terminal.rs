use std::io::{stdout, Write};
use std::time;

use chrono::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, terminal, QueueableCommand};

use crate::util;

pub fn init() {
    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();
    stdout.flush().unwrap();
    terminal::enable_raw_mode().unwrap();
}

pub fn quit() {
    let mut stdout = stdout();
    terminal::disable_raw_mode().unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();
}

pub fn ctrlc() -> bool {
    if event::poll(time::Duration::from_millis(500)).unwrap() {
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn draw_duration(duration: Duration) {
    let duration = util::format_duration(duration);
    let mut stdout = stdout();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    write!(&mut stdout, "{}", duration).unwrap();
    stdout.flush().unwrap();
}
