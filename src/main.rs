use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

// mod file_select;
// use crate::file_select::select_file_path;

// mod parse_file;
// use crate::parse_file::parse_file;

mod alternate_screen;
use alternate_screen::{init_screen, restore_screen};

mod controller;
use controller::start_controller;

mod drawer;
use drawer::clear_and_print;

fn main() -> Result<(), io::Error> {
    let stdout = stdout();

    init_screen(&stdout)?;

    let rx = start_controller();

    'outer: loop {
        while let Ok(c) = rx.try_recv() {
            if c == 'q' {
                break 'outer;
            }
            clear_and_print(&stdout, format!("pressed key: {}", c))?;
        }

        sleep(Duration::from_millis(100));
    }

    restore_screen(&stdout)?;

    // let file_path = select_file_path();
    // println!("Selected file: {}", file_path);
    // let subtitles = parse_file(file_path);

    Ok(())
}
