pub mod model;

pub struct Api<'a> {
    token: &'a str,
}

impl<'a> Api<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub async fn get_player_details(&self, nickname: &str) -> Option<model::PlayerDetails> {
        reqwest::Client::new()
            .get("https://open.faceit.com/data/v4/players")
            .query(&[("game", "csgo"), ("nickname", nickname)])
            .bearer_auth(&self.token)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }

    pub async fn get_player_stats(&self, player_id: &str) -> Option<model::PlayerStats> {
        reqwest::Client::new()
            .get(&format!(
                "https://open.faceit.com/data/v4/players/{}/stats/csgo",
                player_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()
    }
}
