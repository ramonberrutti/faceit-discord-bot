#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDetails {
    #[serde(rename = "player_id")]
    pub player_id: String,
    pub nickname: String,
    pub avatar: String,
    pub country: String,
    #[serde(rename = "cover_image")]
    pub cover_image: String,
    #[serde(rename = "cover_featured_image")]
    pub cover_featured_image: String,
    pub infractions: Infractions,
    pub platforms: Platforms,
    pub games: std::collections::HashMap<String, Games>,
    pub settings: Settings,
    #[serde(rename = "friends_ids")]
    pub friends_ids: Vec<String>,
    pub bans: Vec<::serde_json::Value>,
    #[serde(rename = "new_steam_id")]
    pub new_steam_id: String,
    #[serde(rename = "steam_id_64")]
    pub steam_id64: String,
    #[serde(rename = "steam_nickname")]
    pub steam_nickname: String,
    #[serde(rename = "membership_type")]
    pub membership_type: String,
    pub memberships: Vec<String>,
    #[serde(rename = "faceit_url")]
    pub faceit_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Infractions {
    #[serde(rename = "last_infraction_date")]
    pub last_infraction_date: String,
    pub afk: i64,
    pub leaver: i64,
    #[serde(rename = "qm_not_checkedin")]
    pub qm_not_checkedin: i64,
    #[serde(rename = "qm_not_voted")]
    pub qm_not_voted: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platforms {
    pub steam: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    #[serde(rename = "game_profile_id")]
    pub game_profile_id: String,
    pub region: String,
    pub regions: Option<Regions>,
    #[serde(rename = "skill_level_label")]
    pub skill_level_label: String,
    #[serde(rename = "game_player_id")]
    pub game_player_id: String,
    #[serde(rename = "skill_level")]
    pub skill_level: i64,
    #[serde(rename = "faceit_elo")]
    pub faceit_elo: i64,
    #[serde(rename = "game_player_name")]
    pub game_player_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regions {
    #[serde(rename = "SA")]
    pub sa: Sa,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sa {
    #[serde(rename = "selected_ladder_id")]
    pub selected_ladder_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyale {
    #[serde(rename = "game_profile_id")]
    pub game_profile_id: String,
    pub region: String,
    pub regions: ::serde_json::Value,
    #[serde(rename = "skill_level_label")]
    pub skill_level_label: String,
    #[serde(rename = "game_player_id")]
    pub game_player_id: String,
    #[serde(rename = "skill_level")]
    pub skill_level: i64,
    #[serde(rename = "faceit_elo")]
    pub faceit_elo: i64,
    #[serde(rename = "game_player_name")]
    pub game_player_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub language: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    #[serde(rename = "player_id")]
    pub player_id: String,
    #[serde(rename = "game_id")]
    pub game_id: String,
    pub lifetime: Lifetime,
    pub segments: Vec<Segment>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifetime {
    #[serde(rename = "Total Headshots %")]
    pub total_headshots: String,
    #[serde(rename = "Recent Results")]
    pub recent_results: Vec<String>,
    #[serde(rename = "K/D Ratio")]
    pub k_d_ratio: String,
    #[serde(rename = "Wins")]
    pub wins: String,
    #[serde(rename = "Matches")]
    pub matches: String,
    #[serde(rename = "Current Win Streak")]
    pub current_win_streak: String,
    #[serde(rename = "Average K/D Ratio")]
    pub average_k_d_ratio: String,
    #[serde(rename = "Win Rate %")]
    pub win_rate: String,
    #[serde(rename = "Longest Win Streak")]
    pub longest_win_streak: String,
    #[serde(rename = "Average Headshots %")]
    pub average_headshots: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mode: String,
    pub label: String,
    #[serde(rename = "img_small")]
    pub img_small: String,
    #[serde(rename = "img_regular")]
    pub img_regular: String,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "Average Kills")]
    pub average_kills: String,
    #[serde(rename = "Kills")]
    pub kills: String,
    #[serde(rename = "Assists")]
    pub assists: String,
    #[serde(rename = "Total Headshots %")]
    pub total_headshots: String,
    #[serde(rename = "Win Rate %")]
    pub win_rate: String,
    #[serde(rename = "Deaths")]
    pub deaths: String,
    #[serde(rename = "Penta Kills")]
    pub penta_kills: String,
    #[serde(rename = "Rounds")]
    pub rounds: String,
    #[serde(rename = "Headshots per Match")]
    pub headshots_per_match: String,
    #[serde(rename = "Headshots")]
    pub headshots: String,
    #[serde(rename = "Matches")]
    pub matches: String,
    #[serde(rename = "Average Deaths")]
    pub average_deaths: String,
    #[serde(rename = "K/R Ratio")]
    pub k_r_ratio: String,
    #[serde(rename = "Average MVPs")]
    pub average_mvps: String,
    #[serde(rename = "MVPs")]
    pub mvps: String,
    #[serde(rename = "Quadro Kills")]
    pub quadro_kills: String,
    #[serde(rename = "Average Penta Kills")]
    pub average_penta_kills: String,
    #[serde(rename = "Wins")]
    pub wins: String,
    #[serde(rename = "Average Quadro Kills")]
    pub average_quadro_kills: String,
    #[serde(rename = "Average K/R Ratio")]
    pub average_k_r_ratio: String,
    #[serde(rename = "K/D Ratio")]
    pub k_d_ratio: String,
    #[serde(rename = "Average Triple Kills")]
    pub average_triple_kills: String,
    #[serde(rename = "Average Headshots %")]
    pub average_headshots: String,
    #[serde(rename = "Triple Kills")]
    pub triple_kills: String,
    #[serde(rename = "Average K/D Ratio")]
    pub average_k_d_ratio: String,
    #[serde(rename = "Average Assists")]
    pub average_assists: String,
}
