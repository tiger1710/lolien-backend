mod r#match;

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::r#match::Match;

    #[tokio::test]
    async fn it_works() -> Result<(), reqwest::Error> {
        dotenv::dotenv().ok();
        let riot_token = std::env::var("RIOT_TOKEN").expect("RIOT_TOKEN");
        let client = reqwest::Client::new();
        let res: Match  = client
            .get("https://asia.api.riotgames.com/lol/match/v5/matches/KR_6363178603")
            .header("X-Riot-Token", riot_token)
            .send()
            .await?
            .json()
            .await?;

        println!("{res:?}");

        Ok(())
    }
}
