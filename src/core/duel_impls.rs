use crate::common::desks_datas::DeskData;
use crate::core::action_event::ToClientMessage;
use crate::core::actions::to_client_actions::ToClientAction;
use crate::core::card::Card;
use crate::core::card_info_manager::CARD_INFO_MAP;
use crate::core::duel::{Duel, DuelInfo};
use crate::core::player_info::PlayerInfo;
use bevy::log::{error, info};
use rand::prelude::SliceRandom;
use rand::thread_rng;

impl Duel {
    // 添加玩家
    pub fn add_player(
        &mut self,
        username: String,
        connect_id: u32,
        desk: DeskData,
    ) -> Result<(), String> {
        // 不能两个都存在
        if self.player1.player_info.is_some() && self.player2.player_info.is_some() {
            return Err("Duel already has two player".to_string());
        }
        if self.player1.player_info.is_some() && self.player2.player_info.is_none() {
            self.player2.player_info =
                Some(PlayerInfo::new(username.clone(), connect_id, desk.cards));
            self.player2.load_desk();
            return Ok(());
        }

        if self.player1.player_info.is_none() {
            self.player1.player_info =
                Some(PlayerInfo::new(username.clone(), connect_id, desk.cards));
            self.player1.load_desk();
            return Ok(());
        }
        Err("不可能的分支".to_string())
    }

    pub fn check_is_ready_to_play(&self) -> bool {
        self.player1.player_info.is_some() && self.player2.player_info.is_some()
    }

    pub fn process(&mut self) -> Result<Vec<ToClientMessage>, String> {
        let mut ret = Vec::new();
        if self.check_is_ready_to_play() {
            if self.process_state.belong == 0 {
                info!("对局准备阶段");
                // 发送Field信息
                ret.push(self.to_init_message(1));
                ret.push(self.to_init_message(2));
                // 发送 抽卡
                info!("双方抽卡");
                ret.extend(self.draw(1, 5));
                ret.extend(self.draw(2, 5));
                // 发生回合信息
                // todo
            }
        }
        Ok(ret)
    }

    // 玩家抽卡
    pub fn draw(&mut self, player: usize, num: usize) -> Vec<ToClientMessage> {
        let duel_player = match player {
            1 => &mut self.player1,
            2 => &mut self.player2,
            _ => {
                panic!("Error！")
            }
        };
        let mut ret = Vec::new();
        match duel_player.draw(num) {
            Ok((my, others)) => {
                // 给自己发送 和给对手发送
                ret.push(ToClientMessage {
                    debug_message: "".to_string(),
                    to_connect_id: self.get_connect_id(player),
                    action: ToClientAction::DrawCard(my),
                });
                ret.push(ToClientMessage {
                    debug_message: "".to_string(),
                    to_connect_id: self.get_other_connect_id(player),
                    action: ToClientAction::DrawCard(others),
                });
            }
            Err(_) => {
                // todo其他事件
            }
        }
        ret
    }
}

impl DuelInfo {
    pub fn draw(
        &mut self,
        num: usize,
    ) -> Result<(Vec<Option<String>>, Vec<Option<String>>), String> {
        let mut ret: Vec<Option<String>> = Vec::new();
        let mut ret2: Vec<Option<String>> = Vec::new();
        let hands = &mut self.player_hand_zone.clone().cards.unwrap_or(Vec::new());
        if let Some(desks) = &mut self.player_desk_zone.cards {
            for i in 0..num {
                if let Some(mut card) = desks.pop() {
                    card.card_to_hand();
                    hands.push(card);
                } else {
                    // TODO 玩家游戏失败
                    return Err("无法抽卡失败".to_string());
                }
            }
        } else {
            // TODO 玩家游戏失败
            return Err("无法抽卡失败".to_string());
        }
        self.player_hand_zone.cards = Some(hands.clone());
        Ok((ret, ret2))
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        if let Some(desk) = &mut self.player_desk_zone.cards {
            info!("Desk Shuffled!");
            desk.shuffle(&mut rng);
        }
    }

    pub fn load_desk(&mut self) {
        if let Some(player_info) = self.player_info.clone() {
            let mut desk_vec = Vec::new();
            for card_code in player_info.desks.iter() {
                if let Some(card_info) = CARD_INFO_MAP.get(card_code) {
                    desk_vec.push(Card::build_desk_card(card_info.clone()));
                } else {
                    error!("Card {} not found", card_code);
                }
            }
        }
    }
}
