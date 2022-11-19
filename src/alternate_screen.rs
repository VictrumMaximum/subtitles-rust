use std::io::Write;

use crossterm::{
    cursor::{EnableBlinking, Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn init_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    execute!(
        out,
        EnterAlternateScreen,
        Hide,
        MoveTo(0, 0),
        Print("Press q to exit...")
    )?;
    enable_raw_mode()?;
    Ok(())
}

pub fn restore_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    disable_raw_mode()?;
    execute!(out, EnableBlinking, Show, LeaveAlternateScreen)
}
