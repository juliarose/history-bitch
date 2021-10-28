use reqwest;
use reqwest::header::{ACCESS_CONTROL_ALLOW_ORIGIN, REFERRER_POLICY};
use crate::parser;

pub async fn get_trades() -> Result<Vec<parser::MappedTrade>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;
    let uri = "https://api.steampowered.com/IEconService/GetTradeHistory/v1/?key=KEY&get_descriptions=1&language=en_us&navigating_back=0&max_trades=10";
    let res = client
        .get(uri)
        .fetch_mode_no_cors()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(REFERRER_POLICY, "no-referrer")
        .send()
        .await?
        .text()
        .await?;
    let trades = parser::parse_json(&res)?;
    
    Ok(trades)
}
