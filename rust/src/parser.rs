use serde::{Deserialize, Serialize};
use serde::de::{Deserializer};
use serde_json;
use std::collections::HashMap;
use std::rc::Rc;

pub type Color = Option<String>;

pub type ClassInfoMap = HashMap<(u32, u64, u64), Rc<ClassInfo>>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description {
    pub r#type: String,
    pub value: Option<String>,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub link: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassInfo {
    pub appid: u32,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub classid: u64,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub instanceid: u64,
    pub currency: bool,
    pub name_color: Color,
    pub background_color: Color,
    pub icon_url: String,
    pub icon_url_large: String,
    pub descriptions: Option<Vec<Description>>,
    pub tradable: bool,
    pub actions: Option<Vec<Action>>,
    pub r#type: String,
    pub name: String,
    pub market_name: String,
    pub market_hash_name: String,
    pub commodity: bool,
    pub market_tradable_restriction: u8,
    pub market_marketable_restriction: u8,
    pub marketable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asset {
    pub appid: u32,
    #[serde(deserialize_with = "from_str_to_u32")]
    pub contextid: u32,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub assetid: u64,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub new_assetid: u64,
    #[serde(deserialize_with = "from_str_to_u32")]
    pub amount: u32,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub classid: u64,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub instanceid: u64,
    #[serde(deserialize_with = "from_str_to_u32")]
    pub new_contextid: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trade {
    #[serde(deserialize_with = "from_str_to_u64")]
    pub tradeid: u64,
    #[serde(deserialize_with = "from_str_to_u64")]
    pub steamid_other: u64,
    pub time_init: u32,
    pub status: u8,
    pub assets_received: Option<Vec<Asset>>,
    pub assets_given: Option<Vec<Asset>>,
}

#[derive(Deserialize, Debug)]
pub struct TradesResponse {
    pub more: bool,
    pub trades: Vec<Trade>,
    #[serde(deserialize_with = "map_classinfo")]
    pub descriptions: ClassInfoMap,
}

fn map_classinfo<'de, D>(deserializer: D) -> Result<ClassInfoMap, D::Error>
where
    D: Deserializer<'de>
{
    let mut map = HashMap::new();
    
    for classinfo in Vec::<ClassInfo>::deserialize(deserializer)? {
        map.insert((classinfo.appid, classinfo.classid, classinfo.instanceid), Rc::new(classinfo));
    }
    
    Ok(map)
}

fn from_str_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let int = s.parse::<u32>().unwrap();
    
    Ok(int)
}

fn from_str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let int = s.parse::<u64>().unwrap();
    
    Ok(int)
}

pub fn parse_json(json: &str) -> serde_json::Result<Vec<MappedTrade>> {
    web_sys::console::time_with_label("parse");
    let trades_json: TradesResponse = serde_json::from_str(json)?;
    web_sys::console::time_end_with_label("parse");
    
    Ok(map_descriptions(trades_json))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MappedTrade {
    pub tradeid: u64,
    pub steamid_other: u64,
    pub time_init: u32,
    pub status: u8,
    pub assets_given: Vec<Item>,
    pub assets_received: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub asset: Asset,
    pub classinfo: Option<Rc<ClassInfo>>,
}

fn get_assets(assets_option: Option<Vec<Asset>>, descriptions: &ClassInfoMap) -> Vec<Item> {
    if let Some(assets) = assets_option {
        assets.iter().map(|asset| {
            let mut classinfo = None;
            
            if let Some(description) = descriptions.get(&(asset.appid, asset.classid, asset.instanceid)) {
                classinfo = Some(Rc::clone(&description));
            }
            
            Item {
                asset: asset.to_owned(),
                classinfo,
            }
        }).collect::<Vec<Item>>()
    } else {
        Vec::new()
    }
}

fn map_descriptions(trades_response: TradesResponse) -> Vec<MappedTrade> {
    trades_response.trades.iter().map(|trade| {
        MappedTrade {
            tradeid: trade.tradeid,
            steamid_other: trade.steamid_other,
            time_init: trade.time_init,
            status: trade.status,
            assets_given: get_assets(trade.assets_given.to_owned(), &trades_response.descriptions),
            assets_received: get_assets(trade.assets_received.to_owned(), &trades_response.descriptions),
        }
    }).collect::<Vec<MappedTrade>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case() {
        assert_eq!(1, 1);
    }
}