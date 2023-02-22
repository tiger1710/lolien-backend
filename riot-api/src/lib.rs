pub mod r#match;

pub use r#match::Match;

pub async fn get_match_info(match_id: &str) -> anyhow::Result<Match> {
    let riot_token = std::env::var("RIOT_TOKEN").expect("RIOT_TOKEN");
    let client = reqwest::Client::new();
    Ok(client
        .get(format!(
            "https://asia.api.riotgames.com/lol/match/v5/matches/{match_id}"
        ))
        .header("X-Riot-Token", riot_token)
        .send()
        .await?
        .json()
        .await?)
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::r#match::Match;

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        dotenv::dotenv().ok();
        let riot_token = std::env::var("RIOT_TOKEN").expect("RIOT_TOKEN");
        let client = reqwest::Client::new();
        let _: Match = client
            .get("https://asia.api.riotgames.com/lol/match/v5/matches/KR_6363178603")
            .header("X-Riot-Token", riot_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(())
    }
}
