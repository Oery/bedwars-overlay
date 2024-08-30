use anyhow::Result;
use dotenvy::dotenv;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{self, AppHandle, Manager};

use crate::stats::{PlayerStats, Root};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerChange {
    username: String,
    has_joined: bool,
}

pub enum QueueEvent {
    PlayerJoin(String),
    PlayerQuit(String),
    PlayerList(Vec<String>),
}

fn get_uuid(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");
    let resp = reqwest::blocking::get(&url)?;
    let json: Value = resp.json()?;

    match json["id"].as_str() {
        Some(uuid) => Ok(uuid.to_string()),
        None => Err("No UUID found".into()),
    }
}

fn get_player_stats(uuid: &str) -> Result<PlayerStats, Box<dyn std::error::Error>> {
    dotenv().expect("Failed to read .env file");
    let api_key = std::env::var("HYPIXEL_API_KEY").expect("HYPIXEL_API_KEY not found");
    println!("API KEY: {}", api_key);

    let url = format!(
        "https://api.hypixel.net/player?key={api_key}&uuid={uuid}",
        uuid = uuid,
        api_key = api_key
    );
    let data = match reqwest::blocking::get(&url) {
        Ok(resp) => {
            let json: Value = resp.json()?;
            Ok(json.to_string())
        }
        Err(err) => Err(err),
    };

    let data = data?;
    let root = match serde_json::from_str(&data) {
        Ok(parsed_data) => {
            let root: Root = parsed_data;
            Ok(root)
        }
        Err(err) => Err(err),
    };

    let root = root?;

    let player = match root.player {
        Some(player) => player,
        None => {
            return Err("No player found".into());
        }
    };

    let stats = match player.stats {
        Some(stats) => stats,
        None => {
            return Err("No stats found".into());
        }
    };

    let bedwars = match stats.bedwars {
        Some(bedwars) => bedwars,
        None => {
            return Err("No bedwars stats found".into());
        }
    };
    let username = player.displayname;

    let level = match player.achievements.bedwars_level {
        Some(level) => level,
        None => 0,
    };
    let wins = match bedwars.wins_bedwars {
        Some(wins) => wins,
        None => 0,
    };
    let games_played = match bedwars.games_played_bedwars {
        Some(games_played) => games_played,
        None => 0,
    };
    let final_kills = match bedwars.final_kills_bedwars {
        Some(final_kills) => final_kills,
        None => 0,
    };
    let final_deaths = match bedwars.final_deaths_bedwars {
        Some(final_deaths) => final_deaths,
        None => 0,
    };

    let fkdr = if final_deaths == 0 {
        final_kills.to_string()
    } else {
        (final_kills as f64 / final_deaths as f64).to_string()
    };

    let winrate = if games_played == 0 {
        "0".to_string()
    } else {
        ((wins as f64 / games_played as f64) * 100.0).to_string()
    };

    let new_package_rank = match player.new_package_rank {
        Some(new_package_rank) => new_package_rank,
        None => "NONE".to_string(),
    };

    let mut rank = match new_package_rank.as_str() {
        "MVP_PLUS" => "MVP+",
        "MVP" => "MVP",
        "VIP_PLUS" => "VIP+",
        "VIP" => "VIP",
        "NONE" => "Non",
        _ => "Non",
    };

    let special_rank = match player.rank {
        Some(rank) => rank,
        None => "NONE".to_string(),
    };

    if special_rank == "YOUTUBER" {
        rank = "YOUTUBER";
    }

    let rank_plus_color = match player.rank_plus_color {
        Some(rank_plus_color) => rank_plus_color,
        None => match rank {
            "MVP++" => "RED".to_string(),
            "MVP+" => "RED".to_string(),
            "VIP+" => "YELLOW".to_string(),
            _ => "WHITE".to_string(),
        },
    };

    match player.monthly_package_rank {
        Some(monthly_rank) => {
            if monthly_rank == "SUPERSTAR" {
                rank = "MVP++";
            }
        }
        None => (),
    }

    let player_stats = PlayerStats {
        rank: rank.to_string(),
        username,
        level: level.to_string(),
        fkdr,
        finalkills: final_kills.to_string(),
        winrate,
        wins: wins.to_string(),
        plus_color: rank_plus_color.to_string(),
    };
    dbg!(&player_stats);
    Ok(player_stats)
}

pub fn parse_line(line: &str) -> Option<QueueEvent> {
    if !line.contains("[CHAT]") {
        return None;
    }

    let join_re = Regex::new(r"\[CHAT\] (\w+).+(\d+)/(\d+)").unwrap();

    // [16:59:50] [Client thread/INFO]: [CHAT] lcvejxy has joined (5/8)!
    if let Some(caps) = join_re.captures(line) {
        let username = caps.get(1).map_or("", |m| m.as_str()).to_string();
        return Some(QueueEvent::PlayerJoin(username));
    }

    // [16:59:50] [Client thread/INFO]: [CHAT] lcvejxy has quit!
    let quit_re = Regex::new(r"\[CHAT\] (\w+) has quit!").unwrap();

    if let Some(caps) = quit_re.captures(line) {
        let username = caps.get(1).map_or("", |m| m.as_str()).to_string();
        return Some(QueueEvent::PlayerQuit(username));
    }

    if !line.contains("ONLINE: ") {
        return None;
    }

    // [16:59:49] [Client thread/INFO]: [CHAT] ONLINE: Marticot_, Lightiey, Lampbearer
    if let Some(online_part) = line.split("ONLINE: ").nth(1) {
        let players = online_part
            .split(", ")
            .map(|name| name.to_string())
            .collect();
        return Some(QueueEvent::PlayerList(players));
    }

    return None;

    // if line.contains("[CHAT] ONLINE") {
    //     let _ = app.emit_all("clear-players", ());
    //     let players = extract_players_from_log(&line);
    //     let _ = app.emit_all("set-players", &players);
    //     println!("{}", &line);

    //     for player in players {
    //         let uuid = get_uuid(&player)?;

    //         let _ = app.emit_all(
    //             "update-player",
    //             PlayerStats {
    //                 rank: "Non".to_string(),
    //                 username: player,
    //                 level: "0".to_string(),
    //                 fkdr: "0".to_string(),
    //                 finalkills: "0".to_string(),
    //                 winrate: "0".to_string(),
    //                 wins: "0".to_string(),
    //                 plus_color: "WHITE".to_string(),
    //             },
    //         );

    //         // match get_player_stats(&uuid) {
    //         //     Ok(stats) => {
    //         //         let _ = app.emit_all("update-player", &stats);
    //         //     }
    //         //     Err(err) => {
    //         //         eprintln!("Error: {}", err);
    //         //     }
    //         // }
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //         // println!("UUID: {}", uuid);
    //         // println!("Username: {}", player);
    //     }

    //     return Ok(());
    // }

    // if line.contains("has joined (") {
    //     let player_change = extract_from_player_change(&line);
    //     println!("{}", &line);
    //     if let Some(player_change) = player_change {
    //         let _ = app.emit_all("add-player", &player_change);
    //         let uuid = get_uuid(&player_change.username)?;
    //         // match get_player_stats(&uuid) {
    //         //     Ok(stats) => {
    //         //         let _ = app.emit_all("update-player", &stats);
    //         //     }
    //         //     Err(err) => {
    //         //         eprintln!("Error: {}", err);
    //         //     }
    //         // }
    //         // println!("UUID: {}", uuid);
    //         // println!("Username: {}", player_change.username);
    //         let _ = app.emit_all(
    //             "update-player",
    //             PlayerStats {
    //                 rank: "Non".to_string(),
    //                 username: player_change.username,
    //                 level: "0".to_string(),
    //                 fkdr: "0".to_string(),
    //                 finalkills: "0".to_string(),
    //                 winrate: "0".to_string(),
    //                 wins: "0".to_string(),
    //                 plus_color: "WHITE".to_string(),
    //             },
    //         );
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //     }
    //     return Ok(());
    // }

    // if let Some(player) = parse_player_left(&line) {
    //     return Some(player);
    // }

    // if line.contains("has left (") {
    //     let player_change = extract_from_player_change(&line);

    //     if let Some(player_change) = player_change {
    //         // let _ = app.emit_all("remove-player", player_change);
    //         // println!("Player left : {:?}", player_change);
    //     }
    //     return Ok(());
    // }
    // return None;
}
