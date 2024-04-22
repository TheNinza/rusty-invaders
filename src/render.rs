use crate::frame::Frame;
use crossterm::{
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (y, row) in curr_frame.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != last_frame[y][x] || force {
                stdout
                    .queue(crossterm::cursor::MoveTo(x as u16, y as u16))
                    .unwrap();
                print!("{}", *cell);
            }
        }
    }

    stdout.flush().unwrap();
}
