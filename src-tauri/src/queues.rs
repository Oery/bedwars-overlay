use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::stats::MojangPlayer;

pub type Queue<T> = Arc<Mutex<VecDeque<T>>>;

async fn get_uuid(client: &reqwest::Client, username: &str) -> Result<String> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");
    let resp = client.get(&url).send().await?;
    let player_json: MojangPlayer = resp.json().await?;

    Ok(player_json.id)
}

pub async fn start_mojang_queue(mojang_queue: Queue<String>, hypixel_queue: Queue<String>) {
    let client = reqwest::Client::new();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let maybe_username = {
            let mut queue = mojang_queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(username) = maybe_username {
            println!("Username: {}", username);

            let fetched_uuid = match get_uuid(&client, &username).await {
                Ok(uuid) => uuid,
                Err(_) => {
                    println!("Player: {} - NICKED", username);
                    continue;
                }
            };

            let mut hypixel_queue_lock = hypixel_queue.lock().unwrap();
            hypixel_queue_lock.push_back(fetched_uuid);
        }
    }
}

pub async fn start_hypixel_queue(hypixel_queue: Queue<String>) {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let maybe_uuid = {
            let mut queue = hypixel_queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(uuid) = maybe_uuid {
            println!("UUID: {}", uuid);
        }
    }
}
