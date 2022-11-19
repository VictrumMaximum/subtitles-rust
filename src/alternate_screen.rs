use std::{
    fmt::Display,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{EnableBlinking, Hide, RestorePosition, SavePosition, Show},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub fn alt_screen() -> Result<(), std::io::Error> {
    let stdout = stdout();

    init_screen(&stdout)?;

    show_for_x_seconds(&stdout, "aha", 1)?;
    show_for_x_seconds(&stdout, "hah", 1)?;

    restore_screen(&stdout)?;

    Ok(())
}

fn show_for_x_seconds<T: Display>(
    mut out: impl Write,
    text: T,
    x: u64,
) -> Result<(), std::io::Error> {
    execute!(out, Print(text))?;
    sleep(Duration::from_secs(x));
    clear_screen(out)?;
    Ok(())
}

fn init_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    execute!(out, EnterAlternateScreen, Hide, SavePosition)?;
    enable_raw_mode()
}

fn restore_screen(mut out: impl Write) -> Result<(), std::io::Error> {
    disable_raw_mode()?;
    execute!(out, EnableBlinking, Show, LeaveAlternateScreen)
}

fn clear_screen(mut stdout: impl Write) -> Result<(), std::io::Error> {
    execute!(stdout, RestorePosition, Clear(ClearType::All))
}
