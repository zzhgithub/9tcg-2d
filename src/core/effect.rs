#[derive(Debug, Clone)]
pub struct Effect {
    // 是否需要发动
    pub need_touch: bool,
    // 频率 （一回合一次 ， 同名卡一回合一次）
    // 发动阶段
    // 时间点： 暴露时，攻击时，登场时，当对手发动某个效果时，从卡组进入到弃牌堆时
    // 费用/条件（xxxx 才能发动）
    // 目标过滤器 filter
    // 动作 (target)
}
