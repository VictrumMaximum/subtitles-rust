use std::io;

mod file_select;
use crate::file_select::select_file_path;

mod parse_file;
use crate::parse_file::parse_file;

mod alternate_screen;
use crate::alternate_screen::alt_screen;

fn main() -> Result<(), io::Error> {
    alt_screen()?;
    // let file_path = select_file_path();

    // println!("Selected file: {}", file_path);

    // let subtitles = parse_file(file_path);

    Ok(())
}
