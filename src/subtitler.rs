use crate::controller::{start_controller, Message};
use crate::drawer::{clear_and_print, clear_screen};
use crate::parse_file::Subtitle;

use std::io::Write;
use std::sync::mpsc::Receiver;
use std::{thread::sleep, time::Duration};

pub fn start_subtitles(
    mut out: impl Write,
    subtitles: Vec<Subtitle>,
) -> Result<(), std::io::Error> {
    let rx = start_controller();

    let mut it = subtitles.iter();

    let mut cur = it.next().unwrap();

    sleep(Duration::from_millis(cur.start_ms));

    while let Some(next) = it.next() {
        let text = cur.lines.join("\n");
        clear_and_print(out.by_ref(), text)?;
        let time_until_clear = cur.end_ms - cur.start_ms;

        let msg = get_message(&rx, time_until_clear);
        should_exit(msg) && break;

        clear_screen(out.by_ref())?;

        let time_until_next = next.start_ms - cur.end_ms;

        let msg = get_message(&rx, time_until_next);
        should_exit(msg) && break;

        cur = next;
    }

    // NOTE: this approach is stupid... if a message is received from rx, but it was not an Exit message,
    // the recv_timeout is still triggered and the display time is cut short...

    let text = cur.lines.join("\n");
    let time_until_clear = cur.end_ms - cur.start_ms;
    clear_and_print(out.by_ref(), text)?;

    Ok(())
}

fn get_message(
    rx: &Receiver<Message>,
    timeout: u64,
) -> Result<Message, std::sync::mpsc::RecvTimeoutError> {
    rx.recv_timeout(Duration::from_millis(timeout))
}

fn should_exit(msg_result: Result<Message, std::sync::mpsc::RecvTimeoutError>) -> bool {
    match msg_result {
        Ok(Message::Exit) => true,
        _ => false,
    }
}
