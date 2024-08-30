use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "Bedwars")]
    pub bedwars: Option<Bedwars>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bedwars {
    #[serde(rename = "final_deaths_bedwars")]
    pub final_deaths_bedwars: Option<i64>,
    #[serde(rename = "final_kills_bedwars")]
    pub final_kills_bedwars: Option<i64>,
    #[serde(rename = "wins_bedwars")]
    pub wins_bedwars: Option<i64>,
    #[serde(rename = "losses_bedwars")]
    pub losses_bedwars: Option<i64>,
    #[serde(rename = "games_played_bedwars")]
    pub games_played_bedwars: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievements {
    #[serde(rename = "bedwars_level")]
    pub bedwars_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub success: bool,
    pub player: Option<Player>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub uuid: String,
    pub displayname: String,
    pub stats: Option<Stats>,
    pub monthly_package_rank: Option<String>,
    pub rank_plus_color: Option<String>,
    pub new_package_rank: Option<String>,
    pub rank: Option<String>,
    pub achievements: Achievements,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub rank: String,
    pub username: String,
    pub level: String,
    pub fkdr: String,
    pub finalkills: String,
    pub wins: String,
    pub winrate: String,
    pub plus_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MojangPlayer {
    pub id: String,
    pub name: String,
}
