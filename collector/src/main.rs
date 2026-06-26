//Log ingestion pipeline
mod model;
mod reader;
mod sender;

use std::sync::mpsc;
use std::thread;

use reader::start_reader;
use sender::start_sender;

fn main() {
    let (tx, rx) = mpsc::channel();

    let reader_handle = thread::spawn(move || {
        start_reader(tx);
    });

    start_sender(rx);

    reader_handle.join().unwrap();
}
