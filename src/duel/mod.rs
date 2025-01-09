pub mod main_duel;

use crate::common::game_state::{DuelState, GameState};
use crate::core::action_event::{ActionEvent, ActionType, DuelEvent};
use crate::core::duel::Duel;
use crate::duel::main_duel::{handle_main_button, setup_duel};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

const FPS: usize = 60;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DuelState>();
        app.enable_state_scoped_entities::<DuelState>();
        // 实现游戏对战画面
        app.add_plugins(GgrsPlugin::<DuelEvent>::default());
        app.set_rollback_schedule_fps(FPS);
        // 同步对局资源
        app.rollback_resource_with_clone::<Duel>();
        app.init_resource::<Duel>();
        app.add_systems(GgrsSchedule, deal_events);

        app.insert_resource(NetworkStatsTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        app.add_systems(
            Update,
            print_network_stats_system.run_if(in_state(DuelState::Connected)),
        );

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

#[derive(Resource)]
struct NetworkStatsTimer(Timer);

pub fn deal_events(duel_events: Res<PlayerInputs<DuelEvent>>) {
    for handel in 0..2 {
        match duel_events[handel].0.action {
            ActionType::None => {
                info!("{:?} has no action", duel_events[handel].0.action);
            }
            ActionType::Draw { .. } => {}
            ActionType::Set => {}
            ActionType::Cost => {}
            ActionType::Move => {}
            ActionType::Q => {}
            ActionType::A => {}
            ActionType::Process { .. } => {}
        }
    }
}

// 打印网络连接信息
fn print_network_stats_system(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Option<Res<Session<DuelEvent>>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(sess) = p2p_session {
            match sess.as_ref() {
                Session::P2P(s) => {
                    let num_players = s.num_players();
                    for i in 0..num_players {
                        if let Ok(stats) = s.network_stats(i) {
                            println!("NetworkStats for player {}: {:?}", i, stats);
                        }
                    }
                }
                _ => panic!("This examples focuses on p2p."),
            }
        }
    }
}
