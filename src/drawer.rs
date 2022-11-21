use std::{fmt::Display, io::Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

pub fn clear_and_print<T: Display>(mut out: impl Write, text: T) -> Result<(), std::io::Error> {
    clear_screen(out.by_ref())?;
    execute!(out, Print(text))?;
    Ok(())
}

pub fn clear_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    execute!(out, MoveTo(0, 1), Clear(ClearType::FromCursorDown))
}
