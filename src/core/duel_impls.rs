use crate::common::desks_datas::DeskData;
use crate::core::duel::Duel;

impl Duel {
    // 添加玩家
    pub fn add_player(
        &mut self,
        username: String,
        connect_id: u32,
        desk: DeskData,
    ) -> Result<(), String> {
        // 不能两个都存在
        if self.player1_info.is_some() && self.player2_info.is_some() {
            return Err("Duel already has two player".to_string());
        }
        if self.player1_info.is_some() && self.player2_info.is_none() {
            //TODO 添加到2号位
            return Ok(());
        }

        if self.player1_info.is_none() {
            //TODO 添加到1号位
            return Ok(());
        }
        Err("不可能的分支".to_string())
    }
}
