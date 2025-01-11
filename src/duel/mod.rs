pub mod common;
pub mod main_duel;

use crate::common::game_state::{DuelState, GameState};
use crate::core::action_event::{ToClientMessage, client_register_network_messages};
use crate::duel::main_duel::{handle_main_button, setup_duel};
use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;
use bevy::tasks::futures_lite::StreamExt;
use bevy_eventwork::tcp::NetworkSettings;
use bevy_eventwork::tcp::TcpProvider;
use bevy_eventwork::{EventworkRuntime, NetworkData, NetworkEvent};

const FPS: usize = 60;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DuelState>();
        app.enable_state_scoped_entities::<DuelState>();

        // 连接服务端通用
        app.add_plugins(bevy_eventwork::EventworkPlugin::<
            TcpProvider,
            bevy::tasks::TaskPool,
        >::default());

        app.insert_resource(EventworkRuntime(
            TaskPoolBuilder::new().num_threads(2).build(),
        ));
        client_register_network_messages(app);
        app.insert_resource(NetworkSettings::default());
        // 初始化
        app.add_systems(OnEnter(GameState::Game), setup);
        // 进入到主流程中
        app.add_systems(OnEnter(DuelState::Main), setup_duel);
        app.add_systems(Update, handle_main_button.run_if(in_state(DuelState::Main)));
        // 连接中
        app.add_systems(
            Update,
            handle_network_events.run_if(
                in_state(DuelState::Connecting)
                    .or(in_state(DuelState::Connected))
                    .or(in_state(DuelState::InGame)),
            ),
        );
        // 连接成功
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<DuelState>>) {
    next_state.set(DuelState::Main);
}

fn handle_network_events(
    mut new_network_events: EventReader<NetworkEvent>,
    mut next_duel_state: ResMut<NextState<DuelState>>,
) {
    for event in new_network_events.read() {
        info!("Received event");
        match event {
            NetworkEvent::Connected(_) => {
                info!("Connected !!");
                next_duel_state.set(DuelState::Connected);
            }
            NetworkEvent::Disconnected(_) => {
                info!("Received disconnect");
                next_duel_state.set(DuelState::Main);
            }
            NetworkEvent::Error(error) => {
                error!("Network error: {:?}", error);
                next_duel_state.set(DuelState::ErrorPage);
            }
        }
    }
}

fn handel_message(mut new_messages: EventReader<NetworkData<ToClientMessage>>) {
    for new_message in new_messages.read() {
        info!("Received message {:?}", new_message);
    }
}
