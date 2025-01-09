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
