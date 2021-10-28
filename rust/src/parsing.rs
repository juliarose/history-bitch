use crate::parser;
use crate::requests;

static JSON: &'static str = include_str!("data/response.json");

pub async fn get_trades() -> Result<Vec<parser::MappedTrade>, Box<dyn std::error::Error>> {
    // parser::parse_json(&JSON)
    
    let trades = requests::get_trades().await?;
    
    Ok(trades)
}