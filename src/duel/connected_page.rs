use crate::common::desks_datas::{DeskData, DesksDataList};
use crate::common::game_state::{DuelState, GameState};
use crate::common::settings::Settings;
use crate::core::action_event::ToServerMessage;
use crate::core::actions::to_server_actions::{JoinRoomData, ToServerAction};
use crate::desk::detail::DeskSelect;
use crate::duel::ConnectPlayer;
use crate::duel::common::{create_label_and_input, spawn_button};
use crate::duel::main_duel::{DuelMainAction, DuelMainActionType};
use crate::utils::COLOR_BUTTON;
use bevy::asset::AssetServer;
use bevy::log::info;
use bevy::prelude::{
    AlignItems, BuildChildren, Button, Changed, Commands, Component, FlexDirection, Interaction,
    JustifyContent, NextState, Node, Query, Res, ResMut, Single, StateScoped, Val, With,
};
use bevy::tasks::TaskPool;
use bevy_eventwork::tcp::{NetworkSettings, TcpProvider};
use bevy_eventwork::{EventworkRuntime, Network};
use bevy_persistent::Persistent;
use bevy_simple_text_input::TextInputValue;

#[derive(Component)]
pub struct UsernameInput;
#[derive(Component)]
pub struct RoomNumberInput;

pub fn setup_connected(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            StateScoped(DuelState::Connected),
        ))
        .with_children(|parent| {
            create_label_and_input(parent, font.clone(), "用户名", UsernameInput);
            create_label_and_input(parent, font.clone(), "房间号", RoomNumberInput);

            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "进入游戏",
                DuelMainAction(DuelMainActionType::Start),
            );
            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "断开连接",
                DuelMainAction(DuelMainActionType::Disconnect),
            );
        });
}

pub fn handle_connected_button(
    interaction_query: Query<(&Interaction, &DuelMainAction), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut next_duel_state: ResMut<NextState<DuelState>>,
    // 连接相关
    mut connect_player: ResMut<ConnectPlayer>,
    net: ResMut<Network<TcpProvider>>,
    username_input: Single<&TextInputValue, With<UsernameInput>>,
    room_number_input: Single<&TextInputValue, With<RoomNumberInput>>,
    desk_list: Res<Persistent<DesksDataList>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action.0 {
                DuelMainActionType::Disconnect => {
                    info!("Disconnect by Button!");
                    net.disconnect(connect_player.0.clone().unwrap())
                        .expect("Couldn't disconnect from server!");
                    connect_player.0 = None;
                    next_duel_state.set(DuelState::Main);
                }
                DuelMainActionType::Start => {
                    let list_array: &[DeskData] = &desk_list.list;
                    let i = desk_list.used;
                    let used_desk = list_array[i].clone();
                    let username = username_input.0.clone();
                    let room_number = room_number_input.0.clone();
                    info!(
                        "Username: {} use deskName {} to roomNumber {}",
                        username.clone(),
                        used_desk.name.clone(),
                        room_number.clone()
                    );
                    let data = JoinRoomData {
                        username: username.clone(),
                        room_name: room_number.clone(),
                        desk: used_desk.clone(),
                    };
                    // 尝试发送事件
                    info!("My id: {}", connect_player.0.clone().unwrap().id);
                    net.send_message(connect_player.0.clone().unwrap(), ToServerMessage {
                        debug_message: "".to_string(),
                        action: ToServerAction::JoinRoom(data),
                    })
                    .expect("send message error");
                }
                _ => {
                    info!("unhandled duel action {:?}", action);
                }
            }
        }
    }
}
