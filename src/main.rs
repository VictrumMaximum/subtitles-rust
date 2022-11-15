use std::io;

mod file_select;
use crate::file_select::select_file_path;
mod parse_file;
use crate::parse_file::parse_file;

fn main() -> Result<(), io::Error> {
    let file_path = select_file_path();

    println!("Selected file: {}", file_path);

    let subtitles = parse_file(file_path);

    Ok(())
}
