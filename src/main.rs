use std::io::{self, stdout};

mod drawer;
mod file_select;
mod parse_file;

mod subtitler;
use subtitler::start_subtitles;

mod alternate_screen;
use alternate_screen::{init_screen, restore_screen};

mod controller;

use crate::file_select::select_file_path;
use crate::parse_file::parse_file;

fn main() -> Result<(), io::Error> {
    let out = stdout();

    let file_path = select_file_path();

    let subtitles = parse_file(file_path)?;

    init_screen(&out)?;
    start_subtitles(&out, subtitles)?;
    restore_screen(&out)?;

    Ok(())
}
