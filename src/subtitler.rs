use crate::controller::{start_controller, Message};
use crate::drawer::{clear_and_print_lines, clear_screen};
use crate::parse_file::Subtitle;

use std::io::Write;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn start_subtitles(
    mut out: impl Write,
    subtitles: Vec<Subtitle>,
) -> Result<(), std::io::Error> {
    let rx = start_controller();

    let mut it = subtitles.iter();

    let mut prev: Option<&Subtitle> = None;

    while let Some(cur) = it.next() {
        let time_until_cur = match prev {
            Some(prev_sub) => cur.start_ms - prev_sub.end_ms,
            None => cur.start_ms,
        };

        let _ = sleep_or_exit(&rx, time_until_cur) && break;

        clear_and_print_lines(out.by_ref(), &cur.lines)?;

        let time_until_clear = cur.end_ms - cur.start_ms;
        let _ = sleep_or_exit(&rx, time_until_clear) && break;
        clear_screen(out.by_ref())?;

        prev = Some(cur);
    }

    Ok(())
}

fn sleep_or_exit(rx: &Receiver<Message>, timeout: u64) -> bool {
    match rx.recv_timeout(Duration::from_millis(timeout)) {
        Ok(_) => true,
        _ => false,
    }
}
