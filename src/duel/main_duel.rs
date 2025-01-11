use crate::common::game_state::{DuelState, GameState};
use crate::common::settings::Settings;
use crate::duel::common::spawn_button;
use crate::utils::{BACKGROUND_COLOR, BORDER_COLOR_ACTIVE, COLOR_BUTTON, get_socket};
use bevy::prelude::*;
use bevy::tasks::TaskPool;
use bevy::ui::FocusPolicy;
use bevy_eventwork::tcp::{NetworkSettings, TcpProvider};
use bevy_eventwork::{EventworkRuntime, Network};
use bevy_persistent::Persistent;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Component)]
pub struct DuelMainAction(pub DuelMainActionType);

pub enum DuelMainActionType {
    Connect,
    Disconnect,
    Start,
    Back,
}

pub fn setup_duel(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 创建一个返回的按钮
    // 游戏的场地画面
    // 基本UI
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StateScoped(DuelState::Main),
        ))
        .with_children(|parent| {
            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "建立连接",
                DuelMainAction(DuelMainActionType::Connect),
            );
            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "返回",
                DuelMainAction(DuelMainActionType::Back),
            );
        });
}

pub fn handle_main_button(
    interaction_query: Query<(&Interaction, &DuelMainAction), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_duel_state: ResMut<NextState<DuelState>>,
    // 连接相关
    settings: Res<Persistent<Settings>>,
    net: ResMut<Network<TcpProvider>>,
    network_settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action.0 {
                DuelMainActionType::Connect => {
                    // 尝试建立连接
                    info!(
                        "Starting connect to server {}:{}",
                        settings.service.clone(),
                        settings.port.clone()
                    );
                    net.connect(
                        get_socket(settings.service.clone(), settings.port.clone()),
                        &task_pool.0,
                        &network_settings,
                    );
                    next_duel_state.set(DuelState::Connecting);
                }
                DuelMainActionType::Back => {
                    next_duel_state.set(DuelState::Disable);
                    next_game_state.set(GameState::Menu);
                }
                _ => {
                    info!("Unhandled interaction {:?}", interaction);
                }
            }
        }
    }
}
