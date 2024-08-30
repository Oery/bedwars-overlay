// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::collections::VecDeque;
use std::sync::Arc;

use log_reader::LogReader;
use log_tailer::Tailer;
use queues::start_hypixel_queue;
use queues::start_mojang_queue;
use queues::Queue;
use std::sync::Mutex;
use tauri::App;
use tauri::Manager;

mod event_loop;
mod log_process;
mod log_reader;
mod log_tailer;
mod queues;
mod stats;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            tokio::spawn(async move {
                let hypixel_queue: Queue<String> = Arc::new(Mutex::new(VecDeque::new()));
                let mojang_queue: Queue<String> = Arc::new(Mutex::new(VecDeque::new()));
                let mut tailer = Tailer::new(
                    String::from(r"C:\Users\Oery\AppData\Roaming\.oerymc\logs\latest.log"),
                    &app_handle,
                    mojang_queue.clone(),
                );

                let hypixel_queue_loop = start_hypixel_queue(hypixel_queue.clone());
                let mojang_queue_loop =
                    start_mojang_queue(mojang_queue.clone(), hypixel_queue.clone());
                let tailer_loop = tailer.tail();

                tokio::join!(hypixel_queue_loop, mojang_queue_loop, tailer_loop);
            });

            // tokio::spawn(async move {
            //     futures::executor::block_on(async {
            //         println!("Watching...");
            //         let mut tailer = Tailer::new(
            //             String::from(r"C:\Users\Oery\AppData\Roaming\.oerymc\logs\latest.log"),
            //             &app_handle,
            //         );
            //         tailer.tail().await;
            //         println!("Done watching");
            //     });
            // });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
