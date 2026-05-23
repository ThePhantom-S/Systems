use std::fs::File;
use std::io::{BufReader,BufRead};
use std::thread;
use std::time::Duration;
use serde::Serialize;
use std::time::Instant;


#[derive(Serialize)]
struct LogMessage {
    message:String,
}

fn main(){
    let mut last_flush = Instant::now();
    let file = File::open("app.log").expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let client = reqwest::blocking::Client::new();
    let url = "http://127.0.0.1:8000/logs";
    
    println!("Starting log transferring...to {}",url);
    let mut buffer = Vec::new();

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).unwrap();
    
        if bytes > 0 {
            let trimmed = line.trim().to_string();
            if !trimmed.is_empty() {
                buffer.push(LogMessage { message: trimmed });
                println!("Buffer size before: {}", buffer.len());
            }
        }
    
        // 🔴 ALWAYS CHECK FLUSH — independent of new logs
        if !buffer.is_empty() && (buffer.len() >= 10000 || last_flush.elapsed().as_secs() >= 2) {
            let mut success = false;
    
            for attempt in 0..3 {
                match client.post(url).json(&buffer).send() {
                    Ok(res) if res.status().is_success() => {
                        success = true;
                        break;
                    }
                    Ok(res) => {
                        eprintln!("API error: {}", res.status());
                    }
                    Err(err) => {
                        eprintln!("Request failed: {}", err);
                    }
                }
    
                if attempt < 2 {
                    thread::sleep(Duration::from_millis(500));
                }
            }
    
            println!("Sending batch of size: {}", buffer.len());
    
            if success {
                buffer.clear();
                last_flush = Instant::now();
            }
        }
    
        if bytes == 0 {
            thread::sleep(Duration::from_millis(500));
        }
    }
}