use crate::core::action_event::{ActionEvent, DuelEvent};
use crate::core::duel::Duel;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

const FPS: usize = 60;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        // 实现游戏对战画面
        app.add_plugins(GgrsPlugin::<DuelEvent>::default());
        app.set_rollback_schedule_fps(FPS);
        // 同步对局资源
        app.rollback_resource_with_clone::<Duel>();
        app.init_resource::<Duel>();
    }
}

pub fn setup_duel(mut commands: Commands) {
    // 创建一个返回的按钮
    // 游戏的场地画面
    // 基本UI
}

pub fn deal_events(duel_events: Res<PlayerInputs<DuelEvent>>) {
    for handel in 0..2 {
        match duel_events[handel].0 {}
    }
}
