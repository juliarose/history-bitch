use wasm_bindgen::prelude::*;

mod parser;

pub type ItemNames = Option<Vec<Option<String>>>;

fn parse_trade(trade: &parser::Trade, descriptions: &parser::ClassInfoMap) -> String {
    fn collect_item_names(assets_option: &Option<Vec<parser::Asset>>, descriptions: &parser::ClassInfoMap) -> ItemNames {
        match assets_option {
            Some(assets) => {
                Some(
                    assets.iter().map(|asset| {
                        if let Some(classinfo) = descriptions.get(&(asset.appid, asset.classid, asset.instanceid)) {
                            Some(classinfo.market_hash_name.clone())
                        } else {
                            None
                        }
                    }).collect::<Vec<Option<String>>>()
                )
            },
            None => None,
        }
    }
    
    fn summarize(heading: &str, item_names: Vec<Option<String>>) -> String {
        let mut summary = String::new();
        
        summary.push_str(heading);
        
        let names: String = item_names.iter().map(|a| {
            match a {
                Some(item_name) => item_name.to_owned(),
                None => String::from("Unknown Item"),
            }
        }).collect::<Vec<String>>().join(", ");
        
        summary.push_str("<div class=\"items\">");
        summary.push_str(&names);
        summary.push_str("</div>");
        summary
    }
     
    let tradeid = trade.tradeid.to_string();
    let mut trade_summary = String::new();
    
    trade_summary.push_str(&format!("<div class=\"trade\">Trade #{}", tradeid));
    trade_summary.push_str("<br/></br>");
    
    if let Some(given_item_names) = collect_item_names(&trade.assets_given, descriptions) {
        trade_summary.push_str(&summarize("Given:<br/>", given_item_names));
    }
    
    if let Some(received_item_names) = collect_item_names(&trade.assets_received, descriptions) {
        trade_summary.push_str(&summarize("Received:<br/>", received_item_names));
    }
    
    trade_summary.push_str("</div>");
    trade_summary
}

#[wasm_bindgen]
pub fn parse(json_str: &str) -> String {
    match parser::parse_json(json_str) {
        Ok(trades_result) => {
            let trades = trades_result.trades;
            let descriptions = trades_result.descriptions;
            
            trades.iter().map(|trade| parse_trade(trade, &descriptions)).collect::<Vec<String>>().join("")
        },
        Err(_e) => {
            String::from("no way")
        },
    }
}
