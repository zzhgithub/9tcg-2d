// 本地保存的卡组列表

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct DesksDataList {
    pub list: Vec<DeskData>,
    // 默认使用的索引
    pub used: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeskData {
    pub name: String,
    pub cards: Vec<String>,
}
