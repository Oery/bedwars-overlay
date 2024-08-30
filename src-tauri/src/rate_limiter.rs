use crate::stats::MojangPlayer;
use std::{collections::VecDeque, sync::Arc, time::Duration};
use tauri::{async_runtime::Mutex, AppHandle};

pub struct RateLimiter {
    client: reqwest::Client,
    tauri_handle: AppHandle,
    pub mojang_queue: Arc<MojangQueue>,
}

pub struct MojangQueue {
    queue: Mutex<VecDeque<String>>,
    client: reqwest::Client,
}

impl MojangQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            client: reqwest::Client::new(),
        })
    }

    pub async fn run(self: Arc<Self>) {
        let self_clone = self.clone();
        println!("Spawning mojang queue...");
        tokio::task::spawn(async move {
            println!("Running mojang queue...");
            self_clone.run_loop().await;
        });
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    pub async fn run_loop(&self) {
        println!("Running mojang queue loop...");
        loop {
            self.tick_queue().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn push(&self, username: String) {
        self.queue.lock().await.push_back(username);
    }

    async fn pop(&self) -> Option<String> {
        self.queue.lock().await.pop_front()
    }

    async fn tick_queue(&self) {
        println!("Tick queue...");
        let player = self.pop().await;
        if player == None {
            return;
        }
        let player = player.unwrap();
        let uuid = self.get_uuid(&player).await;

        match uuid {
            Ok(uuid) => println!("Player: {} - UUID: {}", player, uuid),
            Err(_) => println!("Player: {} - NICKED", player),
        }
    }

    async fn get_uuid(&self, username: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.mojang.com/users/profiles/minecraft/{username}",
            username = username
        );
        let resp = self.client.get(&url).send().await?;
        let player: MojangPlayer = resp.json().await?;

        match player.id {
            Some(uuid) => Ok(uuid.to_string()),
            None => Err("No UUID found".into()),
        }
    }
}

impl RateLimiter {
    pub fn new(tauri_handle: &AppHandle) -> Self {
        Self {
            client: reqwest::Client::new(),
            tauri_handle: tauri_handle.clone(),
            mojang_queue: MojangQueue::new(),
        }
    }

    async fn process_request(&self) -> Result<(), reqwest::Error> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        self.fake_request().await?;
        println!("Processing request...");
        Ok(())
    }

    async fn fake_request(&self) -> Result<(), reqwest::Error> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Fake request...");
        Ok(())
    }
}
