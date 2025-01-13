#[derive(Debug, Clone)]
pub struct PlayerInfo {
    // 名称
    pub name: String,
    // 连接信息
    pub connect_id: u32,
    // 初始卡组
    pub desks: Vec<String>,
    // 生命值
    pub lp: usize,
    // 最大生命值
    pub max_lp: usize,
    // 抽卡数目
    pub draw: usize,
    // 是否先攻
    pub go_first: bool,
}

impl PlayerInfo {
    pub fn new(name: String, connect_id: u32, desks: Vec<String>) -> Self {
        Self {
            name,
            connect_id,
            desks,
            lp: 5,
            max_lp: 6,
            draw: 1,
            go_first: false,
        }
    }
}
