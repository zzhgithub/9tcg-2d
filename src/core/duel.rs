use crate::core::filed::{Filed, FiledType};
use crate::core::player_info::PlayerInfo;
use crate::core::process::ProcessState;
use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct Duel {
    // 玩家信息
    pub player1_info: Option<PlayerInfo>,
    pub player2_info: Option<PlayerInfo>,
    // 场地信息
    pub player1_desk_zone: Filed,
    pub player1_hand_zone: Filed,
    pub player1_lx_zone: Filed,
    pub player1_jq_zone: Filed,
    pub player1_battle_zone: [Filed; 5],
    pub player1_safe_zone: [Filed; 5],
    pub player1_drop_zone: Filed,
    //
    pub player2_desk_zone: Filed,
    pub player2_hand_zone: Filed,
    pub player2_lx_zone: Filed,
    pub player2_jq_zone: Filed,
    pub player2_battle_zone: [Filed; 5],
    pub player2_safe_zone: [Filed; 5],
    pub player2_drop_zone: Filed,

    // 流程信息
    pub process_state: ProcessState,
}

impl Default for Duel {
    fn default() -> Self {
        Duel {
            player1_info: None,
            player2_info: None,
            //
            player1_desk_zone: Filed::desk_filed(1),
            player1_hand_zone: Filed::hand_filed(1),
            player1_lx_zone: Filed::lx_filed(1),
            player1_jq_zone: Filed::jq_filed(1),
            player1_battle_zone: Filed::five_filed_with_type(1, FiledType::BattleZone),
            player1_safe_zone: Filed::five_filed_with_type(1, FiledType::SafeZone),
            player1_drop_zone: Filed::drop_filed(1),
            //
            player2_desk_zone: Filed::desk_filed(2),
            player2_hand_zone: Filed::hand_filed(2),
            player2_lx_zone: Filed::lx_filed(2),
            player2_jq_zone: Filed::jq_filed(2),
            player2_battle_zone: Filed::five_filed_with_type(2, FiledType::BattleZone),
            player2_safe_zone: Filed::five_filed_with_type(2, FiledType::SafeZone),
            player2_drop_zone: Filed::drop_filed(2),
            process_state: ProcessState::default(),
        }
    }
}
