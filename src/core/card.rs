use crate::core::card_info::CardInfo;

#[derive(Debug, Clone)]
// 卡片方向
pub enum CardDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone)]
pub enum CardState {
    // 隐身
    Invisible,
    // 嘲讽
    Taunt,
    // 奇袭
    Raid,
    // 中毒 代层数
    Poison(usize),
    // 暴躁
    Unruly,
}

// 卡片
#[derive(Debug, Clone)]
pub struct Card {
    //卡片层级
    z_index: usize,
    // 正面朝上？
    face_up: bool,
    // 对手是否可见?
    can_see: bool,
    // 卡片方向
    card_direction: CardDirection,
    // 状态信息
    card_state: Option<Vec<CardState>>,
    // 卡片信息
    card_info: CardInfo,
}

impl Card {
    // 生成卡组的卡片
    pub fn build_desk_card(card_info: CardInfo) -> Self {
        Self {
            card_info,
            z_index: 0,
            face_up: false,
            can_see: false,
            card_direction: CardDirection::Up,
            card_state: None,
        }
    }

    // 变为手卡卡片
    pub fn card_to_hand(&mut self) {
        self.face_up = true;
        self.can_see = false;
        self.z_index = 0;
        self.card_state = None;
        self.card_direction = CardDirection::Up;
        //fixme: 重置卡片的攻击和效果等
    }
}
