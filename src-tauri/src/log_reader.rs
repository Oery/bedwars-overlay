use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
};

use crate::{
    log_process::{
        parse_line, QueueEvent::PlayerJoin, QueueEvent::PlayerList, QueueEvent::PlayerQuit,
    },
    log_reader,
};
use anyhow::Result;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use futures::channel::mpsc::{channel, Receiver};
use notify::event::EventKind;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Manager};

use futures::SinkExt;
use futures::StreamExt;
use std::fs::metadata;

pub struct LogReader {
    path: String,
    file: File,
    last_position: u64,
    position_to_start: u64,
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(
        move |res| futures::executor::block_on(async { tx.send(res).await.unwrap() }),
        Config::default(),
    )?;

    Ok((watcher, rx))
}

impl LogReader {
    pub fn new(app: &AppHandle) -> Result<Self> {
        // let path = "C:/Users/Oery/.lunarclient/offline/multiver/logs/latest.log";
        let path = "C:/Users/Oery/AppData/Roaming/.oerymc/logs/latest.log";
        let file = File::open(&path).unwrap();
        let last_position = file.metadata()?.len();

        Ok(LogReader {
            path: path.to_string(),
            file,
            last_position,
            position_to_start: 0,
        })
    }

    fn reopen(&mut self) -> Result<()> {
        self.file = File::open(&self.path)?;
        self.last_position = 0;
        Ok(())
    }

    fn read_lines(&mut self) -> Result<()> {
        let current_length = self.file.seek(SeekFrom::End(0))?;

        if current_length < self.last_position {
            return Ok(());
        }

        self.file.seek(SeekFrom::Start(self.position_to_start))?;

        let reader = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(&self.file),
        );

        for line in reader.lines() {
            if let Err(err) = line {
                eprintln!("Error: {}", err);
                continue;
            }

            let line_result = line.unwrap();

            if line_result.contains("[CHAT]") {
                println!("{}", line_result);
            }

            // if let Some(queue_event) = parse_line(&line_result) {
            //     let _ = match queue_event {
            //         PlayerJoin(player) => println!("Player joined: {}", player),
            //         PlayerQuit(player) => println!("Player quit: {}", player),
            //         PlayerList(players) => print!("Players: {:?}", players),
            //     };
            // }

            // self.last_50_lines.push(line_result);

            // if self.last_50_lines.len() > 50 {
            //     self.last_50_lines.remove(0);
            // }
        }

        // self.last_position = current_length;
        self.position_to_start = self.last_position;
        Ok(())
    }

    pub async fn watch(&mut self) -> notify::Result<()> {
        // let (mut watcher, mut rx) = async_watcher()?;

        loop {
            let metadata = std::fs::metadata(&self.path).unwrap();
            let file_size = metadata.len();

            if self.last_position > file_size {
                self.reopen().unwrap();
            }

            if file_size > self.last_position {
                self.last_position = file_size;
                println!("file size: {}", file_size);
                self.read_lines().unwrap();
            }
        }
        // watcher.watch(path, RecursiveMode::NonRecursive)?;

        // while let Some(res) = rx.next().await {
        //     dbg!(&res);
        //     if let Err(err) = res {
        //         eprintln!("Error: {:?}", err);
        //         continue;
        //     }

        //     match res.unwrap().kind {
        //         EventKind::Modify(_) => self.read_lines().unwrap(),
        //         EventKind::Create(_) => self.reopen().unwrap(),
        //         EventKind::Remove(_) => self.reopen().unwrap(),
        //         EventKind::Any => {}
        //         EventKind::Access(_) => {}
        //         _ => {}
        //     }
        // }

        // while let Some(res) = rx.next().await {
        //     match res {
        //         Ok(event) => match event.kind {
        //             notify::EventKind::Create(_) => {
        //                 println!("create");
        //                 self.reopen().unwrap();
        //             }
        //             notify::EventKind::Modify(_) => self.read_lines().unwrap_or(()),
        //             notify::EventKind::Remove(_) => println!("remove"),
        //             notify::EventKind::Access(_) => println!("access"),
        //             notify::EventKind::Other => println!("other"),
        //             notify::EventKind::Any => println!("any"),
        //         },
        //         Err(e) => println!("watch error: {:?}", e),
        //     }
        // }

        Ok(())
    }
}
