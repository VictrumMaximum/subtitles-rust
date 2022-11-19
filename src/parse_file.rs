use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

use regex::{Captures, Regex};

pub fn parse_file(file_path: String) -> Result<Vec<Subtitle>, Error> {
    let mut bufreader = get_bufreader_from_file(file_path);

    let res = parse_lines(&mut bufreader);

    Ok(res)
}

fn get_bufreader_from_file(file_path: String) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}

fn get_bufreader_from_string(input: &str) -> BufReader<&[u8]> {
    BufReader::new(input.as_bytes())
}

fn parse_lines(bufread: &mut dyn BufRead) -> Vec<Subtitle> {
    let mut res = Vec::new();

    let mut lines = bufread.lines();

    while let Some(Ok(x)) = lines.next() {
        if let Ok(_) = x.trim().parse::<u32>() {
            res.push(parse_subtitle_block(&mut lines));
        }
    }

    res
}

fn get_duration_from_timestamps(time: String) -> u64 {
    let re = Regex::new(r"(\d\d):(\d\d):(\d\d),(\d\d\d)").unwrap();

    let caps: Vec<Captures> = re.captures_iter(&time).collect();

    println!("{:?}", caps);

    if caps.len() != 2 {
        panic!("Invalid time stamps");
    }

    // TODO: calculate duration from capture groups

    0
}

fn parse_subtitle_block(input_lines: &mut Lines<&mut dyn BufRead>) -> Subtitle {
    // TODO: Deal with this double unwrap
    let time = input_lines.next().unwrap().unwrap();

    let duration = get_duration_from_timestamps(time);

    let mut sub_lines = Vec::new();

    while let Some(Ok(x)) = input_lines.next() {
        let x = x.trim().to_owned();

        if x.is_empty() {
            break;
        } else {
            sub_lines.push(x);
        }
    }

    Subtitle {
        duration,
        lines: sub_lines,
    }
}

pub struct ParseSubtitleError {
    reason: String,
}

#[derive(PartialEq, Debug)]
pub struct Subtitle {
    duration: u64, // ms
    lines: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::parse_file::Subtitle;

    use super::{get_bufreader_from_string, get_duration_from_timestamps, parse_lines};

    #[test]
    #[ignore]
    fn one_block() {
        let sub_lines = vec![
            String::from("Hello, my name is, NINOOOOOO"),
            String::from("That's not true!"),
        ];
        let input = format!(
            "1
          00:00:00,000 -> 00:01:00,000
          {}
        ",
            sub_lines.join("\n")
        );
        println!("{}", input);
        let res = parse_lines(&mut get_bufreader_from_string(&input));

        assert_eq!(res.len(), 1);

        let expected = Subtitle {
            duration: 60000, // 1 minute in ms
            lines: sub_lines,
        };

        assert_eq!(res[0], expected);
    }
    #[test]
    fn parse_duration() {
        let time = String::from("00:00:00,000 --> 00:01:00,000");

        let res = get_duration_from_timestamps(time);

        let expected = 60000;

        // assert_eq!(res, expected);
    }
}
