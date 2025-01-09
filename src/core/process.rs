use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProcessState {
    // 当前属于那个玩家
    pub belong: usize,
    // 回合数
    pub turn: usize,
    // 当前回合阶段
    pub phase: Phase,
}

impl Default for ProcessState {
    fn default() -> Self {
        Self {
            belong: 0,
            turn: 0,
            phase: Phase::Start,
        }
    }
}

// 回合内阶段
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Start,
    StandBy,
    Draw,
    Main1,
    Battle,
    Main2,
    StormCheck,
    End,
}
