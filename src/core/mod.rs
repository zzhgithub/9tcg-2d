pub mod action_event;
pub mod actions;
pub mod card;
pub mod card_info;
pub mod card_info_manager;
pub mod duel;
pub mod duel_impls;
pub mod effect;
pub mod filed;
pub mod player_info;
pub mod process;

use bevy::prelude::*;

/// 游戏的核心处理逻辑
#[derive(Debug, Clone)]
pub enum Limited {
    Unlimited,
    Max(usize),
}
