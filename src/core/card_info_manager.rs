use crate::core::card_info::{Attr, CardInfo, CardType, Race};
use bevy::utils::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref CARD_INFO_MAP: Mutex<HashMap<String, CardInfo>> = Mutex::new(HashMap::new());
}

pub fn load_all_cards() {
    // todo 初始化这个值？
    load_S001_A_001();
}

fn load_S001_A_001() {
    let mut map = CARD_INFO_MAP.lock().unwrap();
    map.insert("S001-A-001".to_string(), CardInfo {
        code: "S001-A-001".to_string(),
        pre_name: "APPLe".to_string(),
        name: "APPLe".to_string(),
        pre_ack: 1200,
        ack: 1200,
        pre_cost: 1,
        cost: 1,
        card_type: CardType::Actor,
        pre_meme: vec!["星空".to_string(), "物理".to_string()],
        meme: vec!["星空".to_string(), "物理".to_string()],
        pre_race: vec![Race::Awakened],
        race: vec![Race::Awakened],
        pre_attr: vec![Attr::STAR],
        attr: vec![Attr::STAR],
        pre_effects: vec![],
        effects: vec![],
    });
}
