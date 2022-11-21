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

fn parse_lines(bufread: &mut dyn BufRead) -> Vec<Subtitle> {
    let mut res = Vec::new();

    let mut lines = bufread.lines();

    while let Some(Ok(x)) = lines.next() {
        if let Ok(_) = x.trim().replace("\u{feff}", "").parse::<i64>() {
            res.push(parse_subtitle_block(&mut lines));
        }
    }

    res
}

fn get_duration_from_timestamps(input: String) -> (u64, u64) {
    let re = Regex::new(r"(\d\d):(\d\d):(\d\d),(\d\d\d)").unwrap();

    let caps: Vec<Captures> = re.captures_iter(&input).collect();

    let panic_msg = String::from("Invalid time format");

    let parsed_times: Vec<Vec<u64>> = caps
        .iter()
        .map(|cap_gr| {
            cap_gr
                .iter()
                .flatten()
                .filter_map(|x| x.as_str().parse::<u64>().ok())
                .collect()
        })
        .collect();

    if parsed_times.len() != 2 || parsed_times[0].len() != 4 || parsed_times[1].len() != 4 {
        panic!("{}\n{}", panic_msg, input);
    }

    let parsed_times: Vec<u64> = parsed_times
        .iter()
        .map(|time| time[3] + time[2] * 1000 + time[1] * 60000 + time[0] * 60000 * 24)
        .collect();

    (parsed_times[0], parsed_times[1])
}

fn parse_subtitle_block(input_lines: &mut Lines<&mut dyn BufRead>) -> Subtitle {
    // TODO: Deal with this double unwrap
    let time = input_lines.next().unwrap().unwrap();

    let (start_ms, end_ms) = get_duration_from_timestamps(time);

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
        start_ms,
        end_ms,
        lines: sub_lines,
    }
}

#[derive(PartialEq, Debug)]
pub struct Subtitle {
    pub start_ms: u64,
    pub end_ms: u64,
    pub lines: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::parse_file::Subtitle;

    use super::{get_duration_from_timestamps, parse_lines};

    fn get_bufreader_from_string(input: &str) -> BufReader<&[u8]> {
        BufReader::new(input.as_bytes())
    }

    #[test]
    fn one_block() {
        let sub_lines = vec![
            String::from("Hello, my name is, NINOOOOOO"),
            String::from("- That's not true!"),
        ];
        let input = format!(
            "0
          00:01:00,000 -> 00:02:00,000
          {}
        ",
            sub_lines.join("\n")
        );

        let res = parse_lines(&mut get_bufreader_from_string(&input));

        assert_eq!(res.len(), 1);

        let expected = Subtitle {
            start_ms: 60000,
            end_ms: 120000,
            lines: sub_lines,
        };

        assert_eq!(res[0], expected);
    }
    #[test]
    fn parse_duration() {
        let time = String::from("00:01:00,000 --> 00:02:00,000");

        let (start_ms, end_ms) = get_duration_from_timestamps(time);

        assert_eq!(start_ms, 60000);
        assert_eq!(end_ms, 120000);
    }
}
