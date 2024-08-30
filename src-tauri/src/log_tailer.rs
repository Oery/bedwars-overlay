use encoding_rs::*;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek};
use std::thread::sleep;
use std::time::Duration;
use tauri::AppHandle;

use crate::log_process::{
    parse_line, QueueEvent::PlayerJoin, QueueEvent::PlayerList, QueueEvent::PlayerQuit,
};
use crate::queues::Queue;

pub struct Tailer {
    file_path: String,
    file: File,
    last_read: u64,
    mojang_queue: Queue<String>,
}

impl Tailer {
    pub fn new(file_path: String, app: &AppHandle, mojang_queue: Queue<String>) -> Self {
        let file = File::open(&file_path).unwrap();
        let last_read = file.metadata().unwrap().len();

        Self {
            file_path,
            file,
            last_read,
            mojang_queue,
        }
    }

    fn reopen(&mut self) {
        self.file = File::open(&self.file_path).unwrap();
        self.last_read = 0;
    }

    // Read function that return a result
    async fn read(&mut self) -> Result<String, std::io::Error> {
        let current_length = self.file.metadata()?.len();

        if current_length < self.last_read {
            return Ok(String::from(""));
        }

        self.file.seek(std::io::SeekFrom::Start(self.last_read))?;

        let encoding = WINDOWS_1252; // Specify your encoding here
        let mut reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding)) // Specify the encoding here
            .build(&mut self.file);

        let mut buffer = String::new();
        let bufreader = std::io::BufReader::new(reader);
        // print lines one by one
        for line in bufreader.lines() {
            if let Err(err) = line {
                eprintln!("Error: {}", err);
                continue;
            }

            // if let Some(queue_event) = parse_line(&line?) {
            //     let _ = match queue_event {
            //         PlayerJoin(player) => println!("Player joined: {}", player),
            //         PlayerQuit(player) => println!("Player quit: {}", player),
            //         PlayerList(players) => println!("Players: {:?}", players),
            //     };
            // }

            if let Some(queue_event) = parse_line(&line?) {
                let _ = match queue_event {
                    PlayerJoin(player) => {
                        self.mojang_queue.lock().unwrap().push_back(player);
                    }
                    PlayerQuit(player) => println!("Player quit: {}", player),
                    PlayerList(players) => println!("Players: {:?}", players),
                };
            }
        }
        self.last_read = current_length;

        Ok(buffer)
    }

    pub async fn tail(&mut self) -> String {
        println!("Tailing file: {}", self.file_path);
        loop {
            if let Err(err) = self.read().await {
                eprintln!("Error: {}", err);
                self.reopen();
            }

            tokio::time::sleep(Duration::from_millis(10)).await;

            // sleep(Duration::from_millis(10));
        }
    }
}
