// 本地保存的卡组列表

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct DesksDataList {
    pub list: Vec<DeskData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeskData {
    pub name: String,
    pub cards: Vec<String>,
}
