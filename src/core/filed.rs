use crate::core::Limited;
use crate::core::card::Card;

#[derive(Debug, Clone)]
pub struct Filed {
    pub belong: usize,
    // 编号
    pub num: usize,
    // 类型
    pub filed_type: FiledType,
    // 卡片上线
    pub limited: Limited,
    // 卡片列表
    pub cards: Option<Vec<Card>>,
}

#[derive(Debug, Clone)]
pub enum FiledType {
    // 卡组
    DeskZone,
    // 手牌
    HandZone,
    // 棋牌区
    DropZone,
    // 战场
    BattleZone,
    // 安全屋
    SafeZone,
    // 理性区
    LxZone,
    //激情区
    JQZone,
}

impl Filed {
    pub fn drop_filed(belong: usize) -> Self {
        Self {
            belong,
            num: 0,
            filed_type: FiledType::DropZone,
            limited: Limited::Unlimited,
            cards: None,
        }
    }

    pub fn desk_filed(belong: usize) -> Self {
        Filed {
            belong,
            num: 0,
            filed_type: FiledType::DeskZone,
            limited: Limited::Max(60),
            cards: None,
        }
    }

    pub fn hand_filed(belong: usize) -> Self {
        Self {
            belong,
            num: 0,
            filed_type: FiledType::HandZone,
            limited: Limited::Unlimited,
            cards: None,
        }
    }

    pub fn lx_filed(belong: usize) -> Self {
        Self {
            belong,
            num: 0,
            filed_type: FiledType::LxZone,
            limited: Limited::Max(6),
            cards: None,
        }
    }

    pub fn jq_filed(belong: usize) -> Self {
        Self {
            belong,
            num: 0,
            filed_type: FiledType::JQZone,
            limited: Limited::Max(6),
            cards: None,
        }
    }

    pub fn five_filed_with_type(belong: usize, filed_type: FiledType) -> [Self; 5] {
        [
            Self {
                belong,
                num: 1,
                filed_type: filed_type.clone(),
                limited: Limited::Unlimited,
                cards: None,
            },
            Self {
                belong,
                num: 2,
                filed_type: filed_type.clone(),
                limited: Limited::Unlimited,
                cards: None,
            },
            Self {
                belong,
                num: 3,
                filed_type: filed_type.clone(),
                limited: Limited::Unlimited,
                cards: None,
            },
            Self {
                belong,
                num: 4,
                filed_type: filed_type.clone(),
                limited: Limited::Unlimited,
                cards: None,
            },
            Self {
                belong,
                num: 5,
                filed_type: filed_type.clone(),
                limited: Limited::Unlimited,
                cards: None,
            },
        ]
    }
}
