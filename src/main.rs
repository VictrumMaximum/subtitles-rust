use std::{
    fs::{self, DirEntry, File},
    io::{self, BufRead, BufReader, Error, Lines},
    num::ParseIntError,
};

const BASE_DIR: &str = "./data/";

fn main() -> Result<(), io::Error> {
    let file_path = select_file_path();

    println!("Selected file: {}", file_path);

    let subtitles = parse_file(file_path);

    Ok(())
}

fn select_file_path() -> String {
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

fn parse_file(file_path: String) -> Result<Vec<Subtitle>, Error> {
    let file = File::open(file_path).unwrap();
    let buf_reader = BufReader::new(file);

    let mut it = buf_reader.lines();

    let mut res = Vec::new();

    while let Some(Ok(x)) = it.next() {
        if let Ok(index) = x.parse::<u32>() {
            res.push(parse_subtitle_block(&mut it));
        }
    }

    Ok(res)
}

fn parse_subtitle_block(it: &mut Lines<BufReader<File>>) -> Subtitle {
    // TODO: Deal with this double unwrap
    let time = it.next().unwrap().unwrap();

    let duration = 0; // TODO: set correct duration

    let mut lines = Vec::new();

    while let Some(Ok(x)) = it.next() {
        if x.is_empty() {
            break;
        } else {
            lines.push(x);
        }
    }

    Subtitle { duration, lines }
}

struct ParseSubtitleError {
    reason: String,
}

struct Subtitle {
    duration: u64, // ms
    lines: Vec<String>,
}
