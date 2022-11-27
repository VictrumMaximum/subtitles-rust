use std::{fmt::Display, io::Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
    style::{Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn clear_and_print_lines<T: Display>(
    mut out: impl Write,
    lines: &Vec<T>,
) -> Result<(), std::io::Error> {
    clear_screen(out.by_ref())?;

    execute!(out, SetForegroundColor(crossterm::style::Color::Yellow))?;

    for line in lines {
        execute!(out, Print(line), MoveToNextLine(1))?;
    }

    execute!(out, SetForegroundColor(crossterm::style::Color::White))?;

    Ok(())
}

// Moves the cursor to the start position and clears it from cursor downwards.
pub fn clear_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    execute!(out, MoveTo(0, 1), Clear(ClearType::FromCursorDown))
}
