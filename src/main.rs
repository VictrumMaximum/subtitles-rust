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
use crate::parse_file::{parse_file, Subtitle};

fn main() -> Result<(), io::Error> {
    let out = stdout();

    let file_path = select_file_path();
    println!("Selected file: {}", file_path);

    let subtitles = parse_file(file_path)?;

    // println!("{:?}", subtitles.iter().take(5).collect::<Vec<&Subtitle>>());

    init_screen(&out)?;
    start_subtitles(&out, subtitles)?;
    restore_screen(&out)?;

    Ok(())
}
