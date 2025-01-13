use crate::core::card_info::CardInfo;
use bevy::utils::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CARD_INFO_MAP: HashMap<String, CardInfo> = HashMap::new();
}

pub fn load() {
    // todo 初始化这个值？
}
