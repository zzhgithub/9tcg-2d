pub mod main_duel;

use crate::common::game_state::{DuelState, GameState};
use crate::duel::main_duel::{handle_main_button, setup_duel};
use bevy::prelude::*;

const FPS: usize = 60;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DuelState>();
        app.enable_state_scoped_entities::<DuelState>();

        // 初始化
        app.add_systems(OnEnter(GameState::Game), setup);
        // 进入到主流程中
        app.add_systems(OnEnter(DuelState::Main), setup_duel);
        app.add_systems(Update, handle_main_button.run_if(in_state(DuelState::Main)));
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<DuelState>>) {
    next_state.set(DuelState::Main);
}
