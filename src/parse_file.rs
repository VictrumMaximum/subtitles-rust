use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

pub fn parse_file(file_path: String) -> Result<Vec<Subtitle>, Error> {
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

pub struct ParseSubtitleError {
    reason: String,
}

pub struct Subtitle {
    duration: u64, // ms
    lines: Vec<String>,
}
