use std::{
    fs::{self, DirEntry},
    io,
    num::ParseIntError,
};

const BASE_DIR: &str = "./data/";

pub fn select_file_path() -> String {
    let dir_entries: Vec<DirEntry> = fs::read_dir(BASE_DIR)
        .unwrap_or_else(|_| panic!("Error while reading dir"))
        // Ignore faulty DirEntries
        .filter_map(|x| x.ok())
        .collect();

    if dir_entries.is_empty() {
        panic!("There are no files in the data folder!");
    }

    let options = dir_entries
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_str().unwrap().to_owned())
        .enumerate()
        .map(|(i, file_name)| format!("{}. {}", i + 1, file_name))
        .collect::<Vec<String>>()
        .join("\n");

    loop {
        println!("\nChoose a file:\n{}", options);

        let choice = get_number_input();

        match choice {
            Ok(x) => {
                let choice = x;
                if choice < 1 || choice > dir_entries.len() {
                    println!(
                        "Number must be in the range of 1-{}\n",
                        dir_entries.len() + 1
                    );
                    continue;
                }

                return dir_entries
                    .get(choice - 1)
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap()
                    .to_owned();
            }
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        }
    }
}

fn get_number_input() -> Result<usize, ParseIntError> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<usize>()
}
