use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::model::LogMessage;

pub fn start_reader(tx: Sender<LogMessage>) {
    let file = File::open("app.log")
        .expect("Cannot open file");

    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();

        let bytes = reader.read_line(&mut line).unwrap();

        if bytes == 0 {
            thread::sleep(Duration::from_millis(500));
            continue;
        }

        let trimmed = line.trim().to_string();

        if trimmed.is_empty() {
            continue;
        }

        let log = LogMessage {
            message: trimmed,
        };

        if tx.send(log).is_err() {
            eprintln!("Sender disconnected");
            break;
        }
    }
}