use crate::core::process::ProcessState;
use bevy_ggrs::GgrsConfig;
use serde::{Deserialize, Serialize};

pub type DuelEvent = GgrsConfig<ActionEvent>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActionEvent {
    pub action: ActionType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ActionType {
    #[default]
    None,
    // 谁抽了几张卡
    Draw {
        who: usize,
        num: usize,
    },
    // 放置 从某个位置放置了 模因卡
    Set,
    // 费用
    Cost,
    // 卡片从 a 移动了b
    Move,
    // 询问是否要发动效果？
    Q,
    // 询问结果
    A,
    // 流程发生变化
    Process {
        from: ProcessState,
        to: ProcessState,
    },
}

// 谁 目标 操作 内容
