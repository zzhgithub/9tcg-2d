use crate::core::duel::Duel;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use bevy_eventwork::ConnectionId;

#[derive(Clone, Debug)]
pub enum PlayerState {
    Idle,
    InRoom(String),
}

#[derive(Clone, Debug)]
pub struct Player {
    pub connect_id: ConnectionId,
    // 状态
    pub state: PlayerState,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct PlayersManager(pub HashMap<u32, Player>);

// 游戏大厅对象
#[derive(Debug, Clone, Resource, Default)]
pub struct RoomManager(pub HashMap<String, Duel>);
