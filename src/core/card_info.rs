use crate::core::effect::Effect;

#[derive(Debug, Clone)]
pub enum CardType {
    // 人物
    Actor,
    // 神秘术
    Arcane,
    // 模因
    Meme,
}

//属性
#[derive(Debug, Clone)]
pub enum Attr {
    /**
     * 星
     */
    STAR,
    /**
     * 兽
     */
    BEAST,
    /**
     * 木
     */
    PLANT,
    /**
     * 岩
     */
    MINERAL,
    /**
     * 灵
     */
    SPIRIT,
    /**
     * 智
     */
    INTELLECT,
}
#[derive(Debug, Clone)]
pub enum Race {
    /**
     * 神秘学家
     */
    Arcanist,
    /**
     * 超自然者
     */
    Beyond,
    /**
     * 意识唤醒者
     */
    Awakened,
    /**
     * 混血种
     */
    Mixed,
    /**
     * 人类
     */
    Human,
}
#[derive(Debug, Clone)]
// 这里的数据要提前预加载？
pub struct CardInfo {
    // 卡片编号
    pub code: String,
    // 卡片原本名称
    pub pre_name: String,
    // 卡片名称
    pub name: String,

    // 原本攻击力
    pub pre_ack: usize,
    // 攻击力
    pub ack: usize,

    // 费用
    pub pre_cost: usize,
    pub cost: usize,

    // 卡片类型
    pub card_type: CardType,

    // 模因
    pub pre_meme: Vec<String>,
    pub meme: Vec<String>,

    // 种族
    pub pre_race: Vec<Race>,
    pub race: Vec<Race>,

    // 熟悉
    pub pre_attr: Vec<Attr>,
    pub attr: Vec<Attr>,
    //  效果
    pub pre_effects: Vec<Effect>,
    pub effects: Vec<Effect>,
}
