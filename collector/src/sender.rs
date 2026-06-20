use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};

use crate::model::LogMessage;

pub fn start_sender(rx: Receiver<LogMessage>) {
    let client = reqwest::blocking::Client::new();
    let url = "http://localhost:8000/logs";

    let mut buffer: Vec<LogMessage> = Vec::new();

    let batch_size = 20000;
    let mut last_flush = Instant::now();

    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(log) => {
                buffer.push(log);
            }

            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // timeout used for time-based flush
            }

            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("Reader disconnected");
                break;
            }
        }

        let should_flush =
            !buffer.is_empty()
            && (
                buffer.len() >= batch_size
                || last_flush.elapsed().as_secs() >= 2
            );

        if should_flush {
            let mut success = false;

            for attempt in 0..3 {
                match client.post(url).json(&buffer).send() {
                    Ok(response) if response.status().is_success() => {
                        success = true;
                        break;
                    }

                    Ok(response) => {
                        eprintln!(
                            "API error: {}",
                            response.status()
                        );
                    }

                    Err(err) => {
                        eprintln!(
                            "Request failed: {}",
                            err
                        );
                    }
                }

                if attempt < 2 {
                    thread::sleep(
                        Duration::from_millis(500)
                    );
                }
            }

            println!(
                "Sending batch of {} logs",
                buffer.len()
            );

            if success {
                buffer.clear();
                last_flush = Instant::now();
            }
        }
    }
}