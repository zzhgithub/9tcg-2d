use crate::core::duel::Duel;
use crate::core::player_info::PlayerInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ToClientAction {
    InitDuel(InitDuelData),
    DrawCard(Vec<Option<String>>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitDuelData {
    simple_player1: SimplePlayerInfo,
    simple_player2: SimplePlayerInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimplePlayerInfo {
    // 名称
    pub name: String,
    // 连接信息
    pub connect_id: u32,
    // 初始卡组
    pub desks_size: usize,
    // 生命值
    pub lp: usize,
    // 最大生命值
    pub max_lp: usize,
}

impl Duel {
    pub fn to_init_duel(&self) -> InitDuelData {
        InitDuelData {
            simple_player1: self.player1.player_info.clone().unwrap().to_simple_duel(),
            simple_player2: self.player2.player_info.clone().unwrap().to_simple_duel(),
        }
    }
}

impl PlayerInfo {
    pub fn to_simple_duel(&self) -> SimplePlayerInfo {
        SimplePlayerInfo {
            name: self.name.clone(),
            connect_id: self.connect_id.clone(),
            desks_size: self.desks.len(),
            lp: self.lp,
            max_lp: self.max_lp,
        }
    }
}
