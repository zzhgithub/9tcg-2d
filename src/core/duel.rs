use crate::core::filed::{Filed, FiledType};
use crate::core::player_info::PlayerInfo;
use crate::core::process::ProcessState;
use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct Duel {
    pub player1: DuelInfo,
    pub player2: DuelInfo,
    // 流程信息
    pub process_state: ProcessState,
}

impl Default for Duel {
    fn default() -> Self {
        Duel {
            player1: DuelInfo::new(1),
            player2: DuelInfo::new(2),
            process_state: ProcessState::default(),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct DuelInfo {
    pub player_info: Option<PlayerInfo>,
    pub player_desk_zone: Filed,
    pub player_hand_zone: Filed,
    pub player_lx_zone: Filed,
    pub player_jq_zone: Filed,
    pub player_battle_zone: [Filed; 5],
    pub player_safe_zone: [Filed; 5],
    pub player_drop_zone: Filed,
}

impl DuelInfo {
    pub fn new(belong: usize) -> Self {
        Self {
            player_info: None,

            player_desk_zone: Filed::desk_filed(belong),
            player_hand_zone: Filed::hand_filed(belong),
            player_lx_zone: Filed::lx_filed(belong),
            player_jq_zone: Filed::jq_filed(belong),
            player_battle_zone: Filed::five_filed_with_type(belong, FiledType::BattleZone),
            player_safe_zone: Filed::five_filed_with_type(belong, FiledType::SafeZone),
            player_drop_zone: Filed::drop_filed(belong),
        }
    }
}
